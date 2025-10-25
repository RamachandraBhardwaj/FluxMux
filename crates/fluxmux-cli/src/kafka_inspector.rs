use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message as KafkaMessage;
use rdkafka::topic_partition_list::TopicPartitionList;
use rdkafka::Offset;
use std::io::{self, Write};
use tokio::time::Duration;
use futures::StreamExt;
use tokio::signal;

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

    // Force start from beginning for all partitions
    let md = consumer.client().fetch_metadata(Some(topic), Duration::from_secs(5))?;
    let mut tpl = TopicPartitionList::new();
    if let Some(t) = md.topics().iter().find(|t| t.name() == topic) {
        for p in t.partitions() {
            tpl.add_partition_offset(topic, p.id(), Offset::Beginning)?;
        }
        consumer.assign(&tpl)?;
    }

    let mut messages: Vec<Option<String>> = vec![None; n];
    let mut received = 0;

    // Hide cursor
    print!("\x1B[?25l");
    io::stdout().flush()?;

    // Initial display with <nil> placeholders
    render_display(&messages);

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
                        
                        // Move cursor to top and re-render (in-place)
                        print!("\x1B[H"); // Move to home (0,0)
                        render_display(&messages);
                        
                        if received >= n {
                            // Show cursor and exit
                            print!("\x1B[?25h");
                            io::stdout().flush()?;
                            break;
                        }
                    }
                }
                } else if let Some(Err(e)) = maybe_msg {
                    print!("\x1B[?25h"); // Show cursor
                    io::stdout().flush().ok();
                    eprintln!("\nKafka error: {}", e);
                    break;
                }
            }
            _ = signal::ctrl_c() => {
                // Show cursor and exit on Ctrl+C
                print!("\x1B[?25h");
                io::stdout().flush().ok();
                break;
            }
        }
    }

    Ok(())
}

fn render_display(messages: &[Option<String>]) {
    for (i, msg) in messages.iter().enumerate() {
        // Clear current line, then write content
        print!("\x1B[2K"); // Clear entire line
        match msg {
            Some(content) => println!("{}) {}", i + 1, content),
            None => println!("{}) <nil>", i + 1),
        }
    }
    // Clear from cursor to end of screen (removes any leftover lines)
    print!("\x1B[J");
    io::stdout().flush().ok();
}

pub async fn kafka_tail(broker: &str, topic: &str, group: &str, n: usize) -> anyhow::Result<()> {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", group)
        .set("bootstrap.servers", broker)
        .set("enable.auto.commit", "true")
        .set("auto.offset.reset", "latest")
        .set("fetch.min.bytes", "1")
        .set("fetch.wait.max.ms", "50") // Minimal latency
        .set("session.timeout.ms", "6000")
        .set("heartbeat.interval.ms", "2000")
        .create()?;

    consumer.subscribe(&[topic])?;

    // Start from end so we only see new messages
    let md = consumer.client().fetch_metadata(Some(topic), Duration::from_secs(5))?;
    let mut tpl = TopicPartitionList::new();
    if let Some(t) = md.topics().iter().find(|t| t.name() == topic) {
        for p in t.partitions() {
            tpl.add_partition_offset(topic, p.id(), Offset::End)?;
        }
        consumer.assign(&tpl)?;
    }

    // Use a fixed-size array to track the display slots
    let mut messages: Vec<Option<String>> = vec![None; n];
    let mut next_slot = 0; // Next slot to fill
    let mut total_received = 0; // Total messages received

    // Hide cursor
    print!("\x1B[?25l");
    io::stdout().flush()?;

    // Initial display with <nil> placeholders
    render_display(&messages);

    let mut stream = consumer.stream();
    loop {
        tokio::select! {
            maybe_msg = stream.next() => {
                if let Some(Ok(msg)) = maybe_msg {
                    if let Some(payload) = msg.payload() {
                        let text = String::from_utf8_lossy(payload).to_string();
                        
                        // Add message to the current slot (sliding window)
                        messages[next_slot] = Some(text);
                        next_slot = (next_slot + 1) % n; // Move to next slot circularly
                        total_received += 1;

                        // Move cursor to top and re-render (in-place)
                        print!("\x1B[H"); // Move to home (0,0)
                        
                        // If we haven't filled all slots yet, show messages in order
                        // Otherwise, show from next_slot (oldest) to maintain order
                        if total_received <= n {
                            render_display(&messages);
                        } else {
                            // Rearrange to show oldest to newest
                            let mut display: Vec<Option<String>> = Vec::with_capacity(n);
                            for i in 0..n {
                                let idx = (next_slot + i) % n;
                                display.push(messages[idx].clone());
                            }
                            render_display(&display);
                        }
                        
                        io::stdout().flush()?;
                    }
                } else if let Some(Err(e)) = maybe_msg {
                    print!("\x1B[?25h"); // Show cursor
                    io::stdout().flush().ok();
                    eprintln!("\nKafka error: {}", e);
                    break;
                }
            }
            _ = signal::ctrl_c() => {
                break;
            }
        }
    }

    // Show cursor on exit
    print!("\x1B[?25h");
    io::stdout().flush()?;

    Ok(())
}
