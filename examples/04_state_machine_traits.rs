// cargo run --example 04_state_machine_traits

// Implements a finite state machine (FSM) in Rust.
// This example models a process that moves through three states:
//     Validated → Enriched → Persisted
// The transition between states is triggered by an input event (Event::Process).

// This example shows how to create a flexible and type-safe FSM using traits and dynamic dispatch.
// Each state is a struct that implements a shared trait called `State`.
// We use `Box<dyn State>` to hold the current state and allow transitions between different types.
// The main loop continues until the FSM reaches a terminal state.

// This approach make sense if:
//      The states have very different behaviors.
//      You want to allow other crates to add states.
//      You need to store different data in each state.

// There is a simpler and faster approach based on enum and match expression

use std::fmt::Debug;

// Define the possible events the FSM can handle.
// In this simple example, there's only one event: Process.
#[derive(Clone, Debug)]
pub enum FsmEvent {
    Process,
}

// Define the State trait that all states must implement.
// - handle: defines how the state reacts to an event and transitions to the next state.
// - name: returns the name of the current state as a string for logging and comparison.
pub trait FsmState {
    fn process_event(self: Box<Self>, input: FsmEvent) -> Box<dyn FsmState>;
    fn name(&self) -> &'static str;
}

// State: Validated
// When receiving Event::Process, transitions to Enriched.
struct Validated;
impl FsmState for Validated {
    fn process_event(self: Box<Self>, _event: FsmEvent) -> Box<dyn FsmState> {
        println!("State: Validated -> Enriched");
        Box::new(Enriched)
    }

    fn name(&self) -> &'static str {
        "Validated"
    }
}

// State: Enriched
// When receiving Event::Process, transitions to Persisted.
struct Enriched;
impl FsmState for Enriched {
    fn process_event(self: Box<Self>, _event: FsmEvent) -> Box<dyn FsmState> {
        println!("State: Enriched -> Persisted");
        Box::new(Persisted)
    }

    fn name(&self) -> &'static str {
        "Enriched"
    }
}

// State: Persisted
// This is the final state. It returns itself to indicate that no further transitions occur.
struct Persisted;
impl FsmState for Persisted {
    fn process_event(self: Box<Self>, _event: FsmEvent) -> Box<dyn FsmState> {
        println!("State: Persisted (final state reached)");
        self
    }

    fn name(&self) -> &'static str {
        "Persisted"
    }
}

// Runs the state machine starting from the Validated state.
// Repeatedly applies the same event and transitions between states.
// Stops when the FSM reaches a state that does not change (final state).
fn process_event(event: FsmEvent) {
    let mut state: Box<dyn FsmState> = Box::new(Validated);

    loop {
        // Save the current state's name before moving to the next state
        let current_name = state.name();
        let next = state.process_event(event.clone());

        // If the state hasn't changed, we assume we've reached the final state
        if current_name == next.name() {
            println!("Final state: {}", next.name());
            break;
        }

        state = next;
    }
}

fn main() {
    println!("--- Traits-based State Machine Demo ---");
    process_event(FsmEvent::Process);
}
