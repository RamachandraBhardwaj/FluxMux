use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::fs::File;
use tokio::sync::mpsc::Sender;
use fluxmux_core::message::{Message, Format};
use fluxmux_core::traits::Source;
use async_trait::async_trait;
use chrono::Utc;
use tokio::io::AsyncReadExt;
use serde_json::Value;

pub struct FileSource {
    pub path: String,
}

#[async_trait]
impl Source for FileSource {
    async fn start(&mut self, tx: Sender<Message>) -> anyhow::Result<()> {
        // First, try to read the entire file and parse as a single JSON value (array or object).
        // If that fails, fall back to line-by-line NDJSON parsing.
        let mut file = File::open(&self.path).await?;
        let mut buf = String::new();
        file.read_to_string(&mut buf).await?;
        let trimmed = buf.trim();

        // Helper to send a serde_json::Value
        let send_value = |val: Value| async {
            let payload = val.to_string().into_bytes();
            let msg = Message {
                id: None,
                key: None,
                payload,
                format: Some(Format::Json),
                parsed: Some(val),
                timestamp: Utc::now(),
                headers: Default::default(),
                meta: Default::default(),
            };
            tx.send(msg).await
        };

        // Try full JSON parse
        if let Ok(value) = serde_json::from_str::<Value>(trimmed) {
            match value {
                Value::Array(arr) => {
                    for v in arr {
                        send_value(v).await?;
                    }
                    return Ok(());
                }
                other => {
                    send_value(other).await?;
                    return Ok(());
                }
            }
        }

        // Fall back to NDJSON parsing
        let file = File::open(&self.path).await?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        while let Some(line) = lines.next_line().await? {
            let value: serde_json::Value = serde_json::from_str(&line)?;
            let msg = Message {
                id: None,
                key: None,
                payload: line.into_bytes(),
                format: Some(Format::Json),
                parsed: Some(value),
                timestamp: Utc::now(),
                headers: Default::default(),
                meta: Default::default(),
            };
            tx.send(msg).await?;
        }
        Ok(())
    }
}