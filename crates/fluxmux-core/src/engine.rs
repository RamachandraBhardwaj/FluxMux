use crate::traits::{Source, Sink, Processor};
use crate::message::Message;

use tokio::sync::mpsc;

pub async fn run_pipeline(
    mut source: Box<dyn Source>,
    mut processors: Vec<Box<dyn Processor>>,
    mut sink: Box<dyn Sink>,
) -> anyhow::Result<()> {
    let (tx, mut rx) = mpsc::channel::<Message>(1024);

    tokio::spawn(async move {
        if let Err(e) = source.start(tx).await {
            eprintln!("Source stopped with error: {:?}", e);
        }
    });

    while let Some(msg) = rx.recv().await {
        let mut queue = vec![msg];
        for p in processors.iter_mut() {
            let mut next_queue = Vec::new();
            for m in queue {
                let out = p.process(m).await?;
                next_queue.extend(out);
            }
            queue = next_queue;
            if queue.is_empty() {
                break;
            }
        }
        for m in queue {
            sink.send(m).await?;
        }
    }

    sink.flush().await?;
    Ok(())
}
