// cargo run --example 05_state_machine

// Implementation of a finite state machine (FSM) with enums and match
// Same behavior as the version with traits, but simpler and faster

// Explanations of the differences with your original version:
//      No dynamic dispatch: We use an enum to represent all possible states instead of traits and Box<dyn State>. This eliminates the cost of dynamic dispatch.
//      Simpler: The transition logic is encapsulated in a single transition method instead of having a trait implementation for each state.
//      More efficient: Enums in Rust are very efficient, and pattern matching is optimized by the compiler.
//      Direct comparison: States can be compared directly with == since they are all of the same type (the State enum).
//      Less boilerplate: No need to define a separate struct for each state or implement a trait for each one.

// This approach is recommended when:
//      The number of states is known and fixed.
//      All states can be represented in a simple way.
//      You want a simple and efficient solution.

// The previous approach with traits would be more suitable if:
//      The states have very different behaviors.
//      You want to allow other crates to add states.
//      You need to store different data in each state.

// Définition de tous les états possibles
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FsmState {
    Validated,
    Enriched,
    Persisted,
}

#[derive(Debug, Clone, Copy)]
pub enum FsmEvent {
    Process,
}

impl FsmState {
    // Transition d'état basée sur l'événement reçu
    pub fn transition(self, event: FsmEvent) -> Self {
        match (self, event) {
            (FsmState::Validated, FsmEvent::Process) => {
                println!("State: Validated -> Enriched");
                FsmState::Enriched
            }
            (FsmState::Enriched, FsmEvent::Process) => {
                println!("State: Enriched -> Persisted");
                FsmState::Persisted
            }
            (FsmState::Persisted, FsmEvent::Process) => {
                println!("State: Persisted (final state reached)");
                FsmState::Persisted
            }
        }
    }
}

fn process_event(event: FsmEvent) {
    let mut state = FsmState::Validated;

    loop {
        let next_state = state.transition(event);

        if state == next_state {
            println!("Final state: {next_state:?}");
            break;
        }

        state = next_state;
    }
}

fn main() {
    println!("--- Enums-based State Machine Demo ---");
    process_event(FsmEvent::Process);
}
