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
    topic_created: bool,
}

impl KafkaSink {
    pub fn new(brokers: String, topic: String) -> Self {
        Self { brokers, topic, producer: None, topic_created: false }
    }

    fn ensure_producer(&mut self) {
        if self.producer.is_none() {
            let producer: FutureProducer = ClientConfig::new()
                .set("bootstrap.servers", &self.brokers)
                .set("message.timeout.ms", "5000")
                .set("allow.auto.create.topics.enable", "true")
                .create()
                .expect("Producer creation error");
            self.producer = Some(producer);
        }
    }

    async fn ensure_topic_exists(&mut self) -> anyhow::Result<()> {
        if self.topic_created {
            return Ok(());
        }

        // Ensure producer exists first
        self.ensure_producer();

        // Use the producer to check if topic exists via metadata
        let metadata = self.producer.as_ref().unwrap()
            .client()
            .fetch_metadata(Some(&self.topic), Duration::from_secs(5))?;
        let topic_exists = metadata.topics().iter().any(|t| t.name() == self.topic);

        if !topic_exists {
            // Topic doesn't exist, but will be auto-created on first produce
            // Kafka brokers have allow.auto.create.topics.enable=true by default
            // Just mark it as created so we don't check again
        }

        self.topic_created = true;
        Ok(())
    }
}

#[async_trait]
impl Sink for KafkaSink {
    async fn send(&mut self, msg: Message) -> anyhow::Result<()> {
        self.ensure_producer();
        self.ensure_topic_exists().await?;
        
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
