// cargo run --example 04_state_machine

// implements a finite state machine
// The program models a process that goes through three states:
//      Validated → Enriched → Persisted
// Each state handles an input event (here: Event::Process) and determines the next state.

// Show how to implement a flexible and type-safe FSM using traits and dynamic dispatch in Rust.
// The use of Box<dyn Trait> to hold heterogeneous states that implement a common trait.
// A simple event loop to drive transitions until a final state is reached.

use std::fmt::Debug;

// Define an event
// This is the input the state machine reacts to. There's only one kind of event here: Process.
#[derive(Clone, Debug)]
pub enum Event {
    Process,
}

// Define da State trait
// handle: each state defines how it transitions to the next one when receiving an Event.
// name: used to identify the current state by name (for logging and comparison).
// self: Box<Self> is used to take ownership of the state object, allowing it to be replaced by another.
pub trait State {
    fn handle(self: Box<Self>, input: Event) -> Box<dyn State>;
    fn name(&self) -> &'static str;
}

// Concrete states: Validated, Enriched, Persisted
// Each struct implements the State trait:
// State: Validated
// When in Validated, receiving Event::Process transitions to Enriched.
struct Validated;
impl State for Validated {
    fn handle(self: Box<Self>, _event: Event) -> Box<dyn State> {
        println!("State: Validated -> Enriched");
        Box::new(Enriched)
    }

    fn name(&self) -> &'static str {
        "Validated"
    }
}

// State: Enriched
// Transitions to Persisted.
struct Enriched;
impl State for Enriched {
    fn handle(self: Box<Self>, _event: Event) -> Box<dyn State> {
        println!("State: Enriched -> Persisted");
        Box::new(Persisted)
    }

    fn name(&self) -> &'static str {
        "Enriched"
    }
}

// State: Persisted
// No further state after this. Returning self means the machine has reached its final state.
struct Persisted;
impl State for Persisted {
    fn handle(self: Box<Self>, _event: Event) -> Box<dyn State> {
        println!("State: Persisted (final state reached)");
        self
    }

    fn name(&self) -> &'static str {
        "Persisted"
    }
}

// Core state machine logic
// Starts from Validated.
// Loops through state transitions using handle().
// Ends when the state does not change (meaning it reached a terminal state like Persisted).
fn process_event(event: Event) {
    let mut state: Box<dyn State> = Box::new(Validated);

    loop {
        // Store current state's name before moving it
        let current_name = state.name();
        let next = state.handle(event.clone());

        if current_name == next.name() {
            // Final state reached
            println!("Final state: {}", next.name());
            break;
        }

        state = next;
    }
}

fn main() {
    println!("--- State Machine Demo ---");
    // Launches the state machine with one event.
    process_event(Event::Process);
}
