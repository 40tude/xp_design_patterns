// cargo run --example 06_state_machine_comments ./benches/dummy.c

// Counts BYTES inside C-style block comments /* ... */
// Delimiters (/* and */) are NOT counted
// Raw byte scan; UTF-8 is counted per byte (fast and simple)

use std::fs;

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FsmState {
    Code,      // Outside any comment
    Slash,     // Just saw '/'
    Block,     // Inside /* ... */
    BlockStar, // Inside block; previous byte was '*'
}

impl FsmState {
    /// Step the FSM by one byte and return (next_state, bytes_added).
    pub fn transition(self, b: u8) -> (Self, u64) {
        // use State::*;
        match (self, b) {
            // Outside comment
            (FsmState::Code, b'/') => (FsmState::Slash, 0),
            (FsmState::Code, _) => (FsmState::Code, 0),

            // Just saw '/'
            (FsmState::Slash, b'*') => (FsmState::Block, 0), // start of block comment
            (FsmState::Slash, _) => (FsmState::Code, 0),     // false alarm

            // Inside block comment
            (FsmState::Block, b'*') => (FsmState::BlockStar, 0), // maybe closing next
            (FsmState::Block, _) => (FsmState::Block, 1),        // regular byte in body

            // Inside block, previous byte was '*'
            (FsmState::BlockStar, b'/') => (FsmState::Code, 0),      // end of block (delimiters not counted)
            (FsmState::BlockStar, b'*') => (FsmState::BlockStar, 1), // consecutive '*' is still body
            // Otherwise: previous '*' was content (+1) AND current byte (+1)
            (FsmState::BlockStar, _) => (FsmState::Block, 2),
        }
    }
}

fn main() {
    let path = std::env::args().nth(1).expect("Provide the name of a c file.");
    let data = fs::read(&path).expect("Can't read the file.");

    let mut state = FsmState::Code;
    let mut nb_bytes: u64 = 0;

    for &current_byte in &data {
        let (next, add) = state.transition(current_byte);
        nb_bytes += add;
        state = next;
    }

    println!("{nb_bytes}");
}
