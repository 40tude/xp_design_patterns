// cargo run --example 03_observer

// Rust’s Rc<RefCell<T>> and closures make observer pattern readable.
// Great fit for GUI apps, event loops, and message brokers.
// Using Rc<RefCell<...>> allows closures to be shared and mutated even if they're captured in an immutable environment - an idiomatic Rust trick for simulating dynamic callbacks.

use std::cell::RefCell;
use std::rc::Rc;

type Subscriber = Rc<RefCell<dyn FnMut(String)>>;
struct Topic {
    subs: Vec<Subscriber>,
}
impl Topic {
    fn new() -> Self {
        Topic { subs: vec![] }
    }
    fn subscribe(&mut self, callback: Subscriber) {
        self.subs.push(callback);
    }
    fn publish(&mut self, msg: String) {
        for sub in &self.subs {
            sub.borrow_mut()(msg.clone());
        }
    }
}

fn main() {
    // Create a new topic
    let mut topic = Topic::new();

    // Subscriber 1: prints the received message in uppercase
    let sub1: Subscriber = Rc::new(RefCell::new(|msg: String| {
        println!("Subscriber 1 received: {}", msg.to_uppercase());
    }));
    topic.subscribe(sub1);

    // Subscriber 2: prints the received message in lowercase
    let sub2: Subscriber = Rc::new(RefCell::new(|msg: String| {
        println!("Subscriber 2 received: {}", msg.to_lowercase());
    }));
    topic.subscribe(sub2);

    // Publish a message
    topic.publish("Hello Rust World!".to_string());
}
