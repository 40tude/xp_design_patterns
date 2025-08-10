// cargo run --example 05_state_machine_enums

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
#[derive(Debug)]
pub enum FsmState {
    Validated,
    Enriched,
    Persisted,
}

// #[derive(Debug, Clone, Copy)]
pub enum FsmEvent {
    Process,
}

// Implémentation de la FSM
struct Fsm {
    current_state: FsmState,
}

impl Fsm {
    fn new() -> Self {
        Fsm { current_state: FsmState::Validated }
    }

    pub fn process_event(&mut self, event: FsmEvent) {
        match (&self.current_state, event) {
            (FsmState::Validated, FsmEvent::Process) => {
                self.current_state = FsmState::Enriched;
                // println!("State = Validated -> Enriched");
            }
            (FsmState::Enriched, FsmEvent::Process) => {
                self.current_state = FsmState::Persisted;
                // println!("State = Enriched -> Persisted");
            }
            (FsmState::Persisted, FsmEvent::Process) => {
                // println!("State: Persisted (final state reached)");
            }
        }
    }

    fn current_state(&self) -> &FsmState {
        &self.current_state
    }
}

fn main() {
    let mut my_fsm = Fsm::new();
    println!("Initial state: {:?}", my_fsm.current_state());

    my_fsm.process_event(FsmEvent::Process);
    println!("State after one process event: {:?}", my_fsm.current_state());

    my_fsm.process_event(FsmEvent::Process);
    println!("State after 1 process event: {:?}", my_fsm.current_state());

    my_fsm.process_event(FsmEvent::Process);
    println!("State after 2 process events: {:?}", my_fsm.current_state());
}
