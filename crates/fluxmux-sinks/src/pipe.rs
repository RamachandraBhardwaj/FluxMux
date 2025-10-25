use async_trait::async_trait;
use fluxmux_core::message::Message;
use fluxmux_core::traits::Sink;
use tokio::io::{self, AsyncWriteExt};

pub struct PipeSink;

impl PipeSink { 
	pub fn new() -> Self { 
		Self 
	} 
}

#[async_trait]
impl Sink for PipeSink {
	async fn send(&mut self, msg: Message) -> anyhow::Result<()> {
		let mut stdout = io::stdout();
		stdout.write_all(&msg.payload).await?;
		stdout.write_all(b"\n").await?;
		stdout.flush().await?;
		Ok(())
	}

	async fn flush(&mut self) -> anyhow::Result<()> {
		io::stdout().flush().await?;
		Ok(())
	}
}
