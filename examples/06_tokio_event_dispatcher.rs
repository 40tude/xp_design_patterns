// cargo run --example 06_tokio_event_dispatcher

// multiple workers
// A channel is created for each worker.
// main() sends messages in a round-robin fashion to each worker.
// Each worker listens to its own Receiver.
// They all receive a Shutdown at the end, bringing their loop to a clean end.

use tokio::sync::mpsc;

#[derive(Debug)]
enum Message {
    Event(String),
    Shutdown,
}

// Worker logic
async fn start_worker(mut rx: mpsc::Receiver<Message>, id: usize) {
    while let Some(msg) = rx.recv().await {
        match msg {
            Message::Event(data) => {
                println!("[Worker {id}] received: {data}");
            }
            Message::Shutdown => {
                println!("[Worker {id}] shutting down.");
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    const NUM_WORKERS: usize = 3;

    // Create one sender, and a receiver per worker
    let mut handles = vec![];
    let mut senders = vec![];

    for i in 0..NUM_WORKERS {
        let (tx, rx) = mpsc::channel(100);
        senders.push(tx);
        // Spawn each worker with its own receiver
        let handle = tokio::spawn(start_worker(rx, i));
        handles.push(handle);
    }

    // Simulate load balancing: send messages round-robin
    for i in 0..10 {
        let worker_index = i % NUM_WORKERS;
        let msg = Message::Event(format!("Message {i}"));
        senders[worker_index].send(msg).await.unwrap();
    }

    // Send Shutdown to each worker
    for tx in &senders {
        tx.send(Message::Shutdown).await.unwrap();
    }

    // Wait for all workers to finish
    for handle in handles {
        handle.await.unwrap();
    }
}
