use crate::message::Message;
use async_trait::async_trait;

#[async_trait]
pub trait Middleware: Send + Sync {
    async fn handle(&mut self, msg: Message) -> Option<Message>;
}

pub struct MiddlewareChain {
    middlewares: Vec<Box<dyn Middleware>>,
}

impl MiddlewareChain {
    pub fn new() -> Self {
        Self { middlewares: Vec::new() }
    }
    pub fn add(&mut self, mw: Box<dyn Middleware>) {
        self.middlewares.push(mw);
    }
    pub async fn process(&mut self, mut msg: Message) -> Option<Message> {
        for mw in self.middlewares.iter_mut() {
            match mw.handle(msg).await {
                Some(m) => msg = m,
                None => return None,
            }
        }
        Some(msg)
    }
}

// Deduplicator Middleware
use std::collections::HashSet;
pub struct Deduplicator {
    seen: HashSet<Vec<u8>>,
}
impl Deduplicator {
    pub fn new() -> Self {
        Self { seen: HashSet::new() }
    }
}
#[async_trait]
impl Middleware for Deduplicator {
    async fn handle(&mut self, msg: Message) -> Option<Message> {
        if let Some(ref key) = msg.key {
            if self.seen.contains(key) {
                return None;
            }
            self.seen.insert(key.clone());
        }
        Some(msg)
    }
}

// Throttler Middleware
use tokio::time::{sleep, Duration, Instant};
pub struct Throttler {
    last_sent: Instant,
    min_interval: Duration,
}
impl Throttler {
    pub fn new(rate_per_sec: u64) -> Self {
        Self {
            last_sent: Instant::now(),
            min_interval: Duration::from_secs_f64(1.0 / rate_per_sec as f64),
        }
    }
}
#[async_trait]
impl Middleware for Throttler {
    async fn handle(&mut self, msg: Message) -> Option<Message> {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_sent);
        if elapsed < self.min_interval {
            sleep(self.min_interval - elapsed).await;
        }
        self.last_sent = Instant::now();
        Some(msg)
    }
}

// Batcher Middleware with timeout support
pub struct Batcher {
    batch: Vec<Message>,
    batch_size: usize,
    last_flush: Instant,
    timeout: Duration,
}
impl Batcher {
    pub fn new(batch_size: usize, timeout_ms: u64) -> Self {
        Self { 
            batch: Vec::new(), 
            batch_size,
            last_flush: Instant::now(),
            timeout: Duration::from_millis(timeout_ms),
        }
    }
    
    pub fn should_flush(&self) -> bool {
        !self.batch.is_empty() && 
        (self.batch.len() >= self.batch_size || 
         self.last_flush.elapsed() >= self.timeout)
    }
    
    pub fn get_batch(&mut self) -> Vec<Message> {
        self.last_flush = Instant::now();
        std::mem::take(&mut self.batch)
    }
}
#[async_trait]
impl Middleware for Batcher {
    async fn handle(&mut self, msg: Message) -> Option<Message> {
        self.batch.push(msg);
        
        if self.should_flush() {
            // Combine all messages in batch into a single message
            let messages = self.get_batch();
            if messages.is_empty() {
                return None;
            }
            
            // Create a combined message with array payload
            let combined_payload: Vec<serde_json::Value> = messages
                .iter()
                .filter_map(|m| m.parsed.clone())
                .collect();
            
            let payload_json = serde_json::to_vec(&combined_payload).ok()?;
            
            Some(Message {
                id: None,
                key: None,
                payload: payload_json.clone(),
                format: Some(crate::message::Format::Json),
                parsed: serde_json::from_slice(&payload_json).ok(),
                timestamp: chrono::Utc::now(),
                headers: Default::default(),
                meta: Default::default(),
            })
        } else {
            None
        }
    }
}

// RetryHandler Middleware
pub struct RetryHandler {
    max_retries: u32,
    retry_delay: Duration,
}
impl RetryHandler {
    pub fn new(max_retries: u32, delay_ms: u64) -> Self {
        Self {
            max_retries,
            retry_delay: Duration::from_millis(delay_ms),
        }
    }
}
#[async_trait]
impl Middleware for RetryHandler {
    async fn handle(&mut self, msg: Message) -> Option<Message> {
        // RetryHandler is a pass-through middleware
        // Actual retry logic should be in the sink layer
        // This marks the message with retry metadata
        let mut msg = msg;
        msg.meta.insert("max_retries".to_string(), self.max_retries.to_string());
        msg.meta.insert("retry_delay_ms".to_string(), self.retry_delay.as_millis().to_string());
        Some(msg)
    }
}

// SchemaValidator Middleware
use std::path::PathBuf;
use std::fs;

pub struct SchemaValidator {
    schema: Option<serde_json::Value>,
}
impl SchemaValidator {
    pub fn new(schema_path: Option<PathBuf>) -> Self {
        let schema = schema_path.and_then(|path| {
            fs::read_to_string(path).ok()
                .and_then(|content| serde_json::from_str(&content).ok())
        });
        Self { schema }
    }
    
    fn validate_against_schema(&self, value: &serde_json::Value) -> Result<(), String> {
        // Simple validation: check if required fields exist
        if let Some(schema) = &self.schema {
            if let Some(required) = schema.get("required").and_then(|r| r.as_array()) {
                if let Some(obj) = value.as_object() {
                    for field in required {
                        if let Some(field_name) = field.as_str() {
                            if !obj.contains_key(field_name) {
                                return Err(format!("Missing required field: {}", field_name));
                            }
                        }
                    }
                }
            }
            Ok(())
        } else {
            // No schema, pass through
            Ok(())
        }
    }
}
#[async_trait]
impl Middleware for SchemaValidator {
    async fn handle(&mut self, msg: Message) -> Option<Message> {
        if let Some(ref parsed) = msg.parsed {
            match self.validate_against_schema(parsed) {
                Ok(_) => Some(msg),
                Err(err) => {
                    eprintln!("Schema validation failed: {} | Message: {}", 
                        err, 
                        serde_json::to_string(parsed).unwrap_or_else(|_| "unable to serialize".to_string())
                    );
                    None
                }
            }
        } else {
            Some(msg)
        }
    }
}
