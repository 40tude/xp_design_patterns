// cargo run --example 05_tokio_event_dispatcher


// Why It Scales
//      Async backpressure via bounded channels
//      Natural separation of concerns
//      Easy fault isolation — crash one worker, others survive
//      No shared mutable state

// This is a multi-producer, single-consumer (mpsc) channel provided by the Tokio asynchronous library.
// It allows several tasks to produce messages to the same consumer, asynchronously.
use tokio::sync::mpsc;

// This is the type of message the channel will carry:
// Event(String) contains a character string,
// Shutdown indicates that the worker should stop.

enum Message {
    Event(String),
    Shutdown,
}

// This is an asynchronous function that receives messages via the rx receiver:
async fn start_worker(mut rx: mpsc::Receiver<Message>) {
    // recv().await reads a message from the channel (or None if the channel is closed),
    // The while let Some(...) loops until the channel is closed or Message::Shutdown is explicitly encountered.
    while let Some(msg) = rx.recv().await {
        match msg {
            Message::Event(data) => {
                println!("Worker received: {}", data);
            }
            Message::Shutdown => {
                println!("Worker shutting down.");
                break;
            }
        }
    }
}

// This macro initializes the Tokio execution environment. It allows main to be written as an async function.
#[tokio::main]
async fn main() {
    // Creates a channel with 100 buffer messages.
    // tx is the sender (producer), rx the receiver (consumer).
    let (tx, rx) = mpsc::channel(100);

    // Starts the start_worker function in an independent asynchronous task (similar to a light thread).
    tokio::spawn(start_worker(rx));

    // Sends a “hello” message to the worker,
    // Then sends Shutdown to tell it to stop.
    tx.send(Message::Event("hello".into())).await.unwrap();
    tx.send(Message::Shutdown).await.unwrap();
}
