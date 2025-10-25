use fluxmux_core::traits::Source;
use fluxmux_core::message::{Message, Format};
use async_trait::async_trait;
use tokio::sync::mpsc::Sender;
use rdkafka::consumer::{StreamConsumer, Consumer};
use rdkafka::Message as KafkaMessage;
use futures_util::StreamExt;
use chrono::Utc;

pub struct KafkaSource {
    pub brokers: String,
    pub topic: String,
    pub group_id: String,
}

impl KafkaSource {
    pub fn new(brokers: String, topic: String, group_id: String) -> Self {
        Self { brokers, topic, group_id }
    }
}

#[async_trait]
impl Source for KafkaSource {
    async fn start(&mut self, tx: Sender<Message>) -> anyhow::Result<()> {
        let consumer: StreamConsumer = rdkafka::config::ClientConfig::new()
            .set("bootstrap.servers", &self.brokers)
            .set("group.id", &self.group_id)
            .set("auto.offset.reset", "earliest")
            .create()?;

        consumer.subscribe(&[&self.topic])?;

        let mut stream = consumer.stream();
        while let Some(result) = stream.next().await {
            match result {
                Ok(msg) => {
                    if let Some(payload) = msg.payload() {
                        let message = Message {
                            id: None,
                            key: msg.key().map(|k| k.to_vec()),
                            payload: payload.to_vec(),
                            format: Some(Format::Binary),
                            parsed: None,
                            timestamp: Utc::now(),
                            headers: Default::default(),
                            meta: Default::default(),
                        };
                        tx.send(message).await?;
                    }
                }
                Err(e) => {
                    eprintln!("Error processing Kafka message: {}", e);
                    continue;
                }
            }
        }
        Ok(())
    }
}