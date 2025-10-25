use fluxmux_core::traits::Sink;
use fluxmux_core::message::Message;
use async_trait::async_trait;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

pub struct FileSink {
    pub path: String,
    buffer: Vec<Message>,
    buffer_size: usize,
}

impl FileSink {
    pub fn new(path: String, buffer_size: usize) -> Self {
        Self { path, buffer: Vec::with_capacity(buffer_size), buffer_size }
    }
}

#[async_trait]
impl Sink for FileSink {
    async fn send(&mut self, msg: Message) -> anyhow::Result<()> {
        self.buffer.push(msg);
        if self.buffer.len() >= self.buffer_size {
            self.flush().await?;
        }
        Ok(())
    }

    async fn flush(&mut self) -> anyhow::Result<()> {
        if self.buffer.is_empty() {
            return Ok(());
        }
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.path)
            .await?;
        for msg in self.buffer.drain(..) {
            file.write_all(&msg.payload).await?;
            file.write_all(b"\n").await?;
        }
        file.flush().await?;
        Ok(())
    }
}