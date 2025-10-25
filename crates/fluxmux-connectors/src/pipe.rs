use async_trait::async_trait;
use chrono::Utc;
use fluxmux_core::message::{Format, Message};
use fluxmux_core::traits::Source;
use serde_json::Value;
use tokio::io::{self, AsyncBufReadExt, AsyncReadExt, BufReader};
use tokio::sync::mpsc::Sender;

pub struct PipeSource;

impl PipeSource {
	pub fn new() -> Self {
		Self
	}
}

#[async_trait]
impl Source for PipeSource {
	async fn start(&mut self, tx: Sender<Message>) -> anyhow::Result<()> {
		let mut stdin = io::stdin();
		let mut buf = String::new();
		stdin.read_to_string(&mut buf).await?;
		let trimmed = buf.trim();

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

		if !trimmed.is_empty() {
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
		}

		// Fallback to line-by-line NDJSON
		let stdin = io::stdin();
		let reader = BufReader::new(stdin);
		let mut lines = reader.lines();
		while let Some(line) = lines.next_line().await? {
			if line.trim().is_empty() { continue; }
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
