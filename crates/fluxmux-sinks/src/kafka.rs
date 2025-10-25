use fluxmux_core::traits::Sink;
use fluxmux_core::message::Message;
use async_trait::async_trait;
use rdkafka::producer::{FutureProducer, FutureRecord, Producer};
use rdkafka::config::ClientConfig;
use std::time::Duration;

pub struct KafkaSink {
    pub brokers: String,
    pub topic: String,
    producer: Option<FutureProducer>,
}

impl KafkaSink {
    pub fn new(brokers: String, topic: String) -> Self {
        Self { brokers, topic, producer: None }
    }

    fn ensure_producer(&mut self) {
        if self.producer.is_none() {
            let producer: FutureProducer = ClientConfig::new()
                .set("bootstrap.servers", &self.brokers)
                .set("message.timeout.ms", "5000")
                .create()
                .expect("Producer creation error");
            self.producer = Some(producer);
        }
    }
}

#[async_trait]
impl Sink for KafkaSink {
    async fn send(&mut self, msg: Message) -> anyhow::Result<()> {
        self.ensure_producer();
        
        let topic = self.topic.clone();
        let record = FutureRecord::to(&topic)
            .payload(&msg.payload)
            .key(msg.key.as_deref().unwrap_or_default());

        match self.producer.as_ref().unwrap().send(record, Duration::from_secs(5)).await {
            Ok(_) => Ok(()),
            Err((e, _)) => Err(anyhow::anyhow!("Failed to send message: {}", e))
        }
    }

    async fn flush(&mut self) -> anyhow::Result<()> {
        if let Some(producer) = &self.producer {
            producer.flush(Duration::from_secs(5))?;
        }
        Ok(())
    }
}
