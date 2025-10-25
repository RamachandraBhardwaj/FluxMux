use crate::message::Message;
use crate::pipe_actions::PipeAction;
use crate::traits::{Sink, Source};
use tokio::sync::mpsc;

pub async fn run_pipe(
    mut source: Box<dyn Source>,
    mut actions: Vec<Box<dyn PipeAction>>,
    mut sinks: Vec<Box<dyn Sink>>,
) -> anyhow::Result<()> {
    let (tx, mut rx) = mpsc::channel::<Message>(1024);

    // Start source in background
    let source_handle = tokio::spawn(async move {
        source.start(tx).await
    });

    // Process messages through action chain
    while let Some(msg) = rx.recv().await {
        let mut messages = vec![msg];

        // Apply each action in sequence
        for action in &mut actions {
            let mut next_messages = vec![];
            for m in messages {
                match action.execute(m).await {
                    Ok(mut results) => next_messages.append(&mut results),
                    Err(e) => eprintln!("Action error: {}", e),
                }
            }
            messages = next_messages;
        }

        // Send to all sinks
        for msg in messages {
            for sink in &mut sinks {
                if let Err(e) = sink.send(msg.clone()).await {
                    eprintln!("Sink error: {}", e);
                }
            }
        }
    }

    // Finalize actions (for aggregates, etc.)
    for action in &mut actions {
        match action.finalize().await {
            Ok(final_messages) => {
                for msg in final_messages {
                    for sink in &mut sinks {
                        if let Err(e) = sink.send(msg.clone()).await {
                            eprintln!("Sink error during finalize: {}", e);
                        }
                    }
                }
            }
            Err(e) => eprintln!("Finalize error: {}", e),
        }
    }

    // Flush all sinks
    for sink in &mut sinks {
        if let Err(e) = sink.flush().await {
            eprintln!("Flush error: {}", e);
        }
    }

    // Wait for source to complete
    source_handle.await??;

    Ok(())
}
