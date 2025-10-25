use crate::middleware::{MiddlewareChain, Deduplicator, Throttler, Batcher, RetryHandler, SchemaValidator};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct MiddlewareConfig {
    pub batch_size: Option<usize>,
    pub batch_timeout_ms: Option<u64>,
    pub deduplicate: Option<bool>,
    pub throttle_per_sec: Option<u64>,
    pub retry_max_attempts: Option<u32>,
    pub retry_delay_ms: Option<u64>,
    pub schema_path: Option<String>,
}

pub fn build_middleware_chain(cfg: &MiddlewareConfig) -> MiddlewareChain {
    let mut chain = MiddlewareChain::new();
    
    // Add SchemaValidator first to validate incoming messages
    if let Some(ref schema_path) = cfg.schema_path {
        chain.add(Box::new(SchemaValidator::new(Some(PathBuf::from(schema_path)))));
    }
    
    // Add Deduplicator
    if cfg.deduplicate.unwrap_or(false) {
        chain.add(Box::new(Deduplicator::new()));
    }
    
    // Add RetryHandler
    if let Some(max_retries) = cfg.retry_max_attempts {
        let delay = cfg.retry_delay_ms.unwrap_or(1000);
        chain.add(Box::new(RetryHandler::new(max_retries, delay)));
    }
    
    // Add Batcher
    if let Some(batch_size) = cfg.batch_size {
        let timeout = cfg.batch_timeout_ms.unwrap_or(5000);
        chain.add(Box::new(Batcher::new(batch_size, timeout)));
    }
    
    // Add Throttler last to control output rate
    if let Some(rate) = cfg.throttle_per_sec {
        chain.add(Box::new(Throttler::new(rate)));
    }
    
    chain
}

use crate::traits::{Source, Sink};
use crate::message::Message;
use tokio::sync::mpsc;

pub async fn run_pipeline(
    mut source: Box<dyn Source>,
    mut middlewares: MiddlewareChain,
    mut sink: Box<dyn Sink>,
) -> anyhow::Result<()> {
    let (tx, mut rx) = mpsc::channel::<Message>(1024);

    let source_handle = tokio::spawn(async move {
        if let Err(e) = source.start(tx).await {
            eprintln!("Source stopped with error: {:?}", e);
        }
    });

    while let Some(msg) = rx.recv().await {
        if let Some(processed_msg) = middlewares.process(msg).await {
            // Retry logic based on message metadata
            let max_retries = processed_msg.meta.get("max_retries")
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(0);
            let retry_delay_ms = processed_msg.meta.get("retry_delay_ms")
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(1000);
            
            let mut attempt = 0;
            loop {
                match sink.send(processed_msg.clone()).await {
                    Ok(_) => break,
                    Err(e) => {
                        if attempt >= max_retries {
                            eprintln!("Failed to send message after {} attempts: {}", max_retries + 1, e);
                            break;
                        }
                        attempt += 1;
                        eprintln!("Retry attempt {}/{} after error: {}", attempt, max_retries, e);
                        tokio::time::sleep(tokio::time::Duration::from_millis(retry_delay_ms)).await;
                    }
                }
            }
        }
    }
    
    sink.flush().await?;
    source_handle.await?;
    Ok(())
}
