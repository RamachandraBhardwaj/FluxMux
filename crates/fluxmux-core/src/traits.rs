use crate::message::Message;
use async_trait::async_trait;
use tokio::sync::mpsc;

#[async_trait]
pub trait Source: Send + Sync {
    async fn start(&mut self, out: mpsc::Sender<Message>) -> anyhow::Result<()>;
}

#[async_trait]
pub trait Sink: Send + Sync {
    async fn send(&mut self, msg: Message) -> anyhow::Result<()>;
    async fn flush(&mut self) -> anyhow::Result<()>;
}

#[async_trait]
pub trait Processor: Send + Sync {
    async fn process(&mut self, msg: Message) -> anyhow::Result<Vec<Message>>;
}
#[async_trait]
pub trait Codec: Send + Sync {
    async fn decode(&self, payload: &[u8]) -> anyhow::Result<serde_json::Value>;
    async fn encode(&self, v: &serde_json::Value) -> anyhow::Result<Vec<u8>>;
}
