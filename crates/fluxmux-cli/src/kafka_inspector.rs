use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message as KafkaMessage;
use rdkafka::topic_partition_list::TopicPartitionList;
use rdkafka::Offset;
use tokio::time::Duration;
use futures::StreamExt;
use tokio::signal;
use std::io::{self, Write};

pub async fn kafka_head(broker: &str, topic: &str, group: &str, n: usize) -> anyhow::Result<()> {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", group)
        .set("bootstrap.servers", broker)
        .set("enable.auto.commit", "false")
        .set("auto.offset.reset", "earliest")
        .set("fetch.min.bytes", "1")
        .set("fetch.wait.max.ms", "100")
        .create()?;

    consumer.subscribe(&[topic])?;

    // Verify topic exists
    let md = consumer.client().fetch_metadata(Some(topic), Duration::from_secs(5))?;
    if md.topics().iter().find(|t| t.name() == topic).is_none() {
        return Err(format!("Topic '{}' does not exist", topic).into());
    }

    // Force start from beginning for all partitions
    let mut tpl = TopicPartitionList::new();
    if let Some(t) = md.topics().iter().find(|t| t.name() == topic) {
        for p in t.partitions() {
            tpl.add_partition_offset(topic, p.id(), Offset::Beginning)?;
        }
        consumer.assign(&tpl)?;
    }

    // Initialize display with <nil> placeholders
    let mut messages: Vec<Option<String>> = vec![None; n];
    let mut received = 0;
    let mut first_print = true;

    let mut stream = consumer.stream();
    loop {
        tokio::select! {
            maybe_msg = stream.next() => {
                if let Some(Ok(msg)) = maybe_msg {
                    if received < n {
                        if let Some(payload) = msg.payload() {
                            let text = String::from_utf8_lossy(payload).to_string();
                            messages[received] = Some(text);
                            received += 1;
                            
                            // Move cursor up n lines to overwrite previous output (not on first print)
                            if !first_print {
                                print!("\x1B[{}A", n);  // Move up N lines at once
                                io::stdout().flush()?;
                            }
                            
                            // Print updated display (overwrite in place)
                            render_display(&messages);
                            io::stdout().flush()?;
                            first_print = false;
                            
                            if received >= n {
                                return Ok(());
                            }
                        }
                    }
                } else if let Some(Err(e)) = maybe_msg {
                    eprintln!("\nKafka error: {}", e);
                    return Err(e.into());
                }
            }
            _ = signal::ctrl_c() => {
                return Ok(());
            }
        }
    }
}

fn render_display(messages: &[Option<String>]) {
    for (i, msg) in messages.iter().enumerate() {
        match msg {
            Some(content) => print!("{}) {}\r\n", i + 1, content),
            None => print!("{}) <nil>\r\n", i + 1),
        }
    }
}

pub async fn kafka_tail(broker: &str, topic: &str, group: &str, n: usize) -> anyhow::Result<()> {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", group)
        .set("bootstrap.servers", broker)
        .set("enable.auto.commit", "true")
        .set("auto.offset.reset", "latest")
        .set("fetch.min.bytes", "1")
        .set("fetch.wait.max.ms", "50")
        .set("session.timeout.ms", "6000")
        .set("heartbeat.interval.ms", "2000")
        .create()?;

    consumer.subscribe(&[topic])?;

    // Verify topic exists
    let md = consumer.client().fetch_metadata(Some(topic), Duration::from_secs(5))?;
    if md.topics().iter().find(|t| t.name() == topic).is_none() {
        return Err(format!("Topic '{}' does not exist", topic).into());
    }

    // Start from end to see only new messages
    let mut tpl = TopicPartitionList::new();
    if let Some(t) = md.topics().iter().find(|t| t.name() == topic) {
        for p in t.partitions() {
            tpl.add_partition_offset(topic, p.id(), Offset::End)?;
        }
        consumer.assign(&tpl)?;
    }

    // Initialize display with <nil> placeholders
    let mut messages: Vec<Option<String>> = vec![None; n];
    let mut next_slot = 0;  // Next slot to fill (for circular buffer)
    let mut total_received = 0; // Total messages ever received
    let mut first_print = true; // Track if this is first print

    let mut stream = consumer.stream();
    loop {
        tokio::select! {
            maybe_msg = stream.next() => {
                if let Some(Ok(msg)) = maybe_msg {
                    if let Some(payload) = msg.payload() {
                        let text = String::from_utf8_lossy(payload).to_string();
                        
                        // Add to circular buffer (replaces <nil> or oldest message)
                        messages[next_slot] = Some(text);
                        next_slot = (next_slot + 1) % n;
                        total_received += 1;

                        // Move cursor up n lines and clear them (not on first print)
                        if !first_print {
                            // Move up N lines and clear all of them
                            for _ in 0..n {
                                print!("\x1B[1A"); // Move up 1 line
                                print!("\x1B[2K"); // Clear that line
                            }
                            print!("\r"); // Ensure we're at column 0
                            io::stdout().flush()?;
                        }
                        
                        // Print updated display (overwrite in place)
                        if total_received <= n {
                            render_display(&messages);
                        } else {
                            // Rearrange to show messages in chronological order
                            let mut display: Vec<Option<String>> = Vec::with_capacity(n);
                            for i in 0..n {
                                let idx = (next_slot + i) % n;
                                display.push(messages[idx].clone());
                            }
                            render_display(&display);
                        }
                        
                        io::stdout().flush()?;
                        first_print = false;
                    }
                } else if let Some(Err(e)) = maybe_msg {
                    eprintln!("\nKafka error: {}", e);
                    return Err(e.into());
                }
            }
            _ = signal::ctrl_c() => {
                return Ok(());
            }
        }
    }
}
