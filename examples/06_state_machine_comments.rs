// cargo run --example 06_state_machine_comments ./benches/dummy.c

// Counts BYTES inside C-style block comments /* ... */
// Delimiters (/* and */) are NOT counted
// Raw byte scan; UTF-8 is counted per byte (fast and simple)

const BYTE_NOT_COUNTED: u64 = 0;
const ONE_BYTE_COUNTED: u64 = 1;
const TWO_BYTES_COUNTED: u64 = 2;

use std::fs;

#[derive(Debug)]
enum FsmState {
    Code,      // Outside any comment
    Slash,     // Just saw '/'
    Block,     // Inside /* ... */
    BlockStar, // Inside block; previous byte was '*'
}

// Here the events are the bytes read in the `.c` file
// pub enum FsmEvent {
//     Process,
// }

struct Fsm {
    current_state: FsmState,
}

impl Fsm {
    fn new() -> Self {
        Fsm { current_state: FsmState::Code }
    }

    pub fn process_byte(&mut self, b: u8) -> u64 {
        // match only the current state to help potential optimization
        // No longer `match (&self.current_state, b)`
        match self.current_state {
            FsmState::Code => self.process_code(b),
            FsmState::Slash => self.process_slash(b),
            FsmState::Block => self.process_comment(b),
            FsmState::BlockStar => self.process_star(b),
        }
    }

    // Outside any comment
    fn process_code(&mut self, b: u8) -> u64 {
        if b == b'/' {
            // potential comment start
            self.current_state = FsmState::Slash;
            BYTE_NOT_COUNTED
        } else {
            // stay in code state
            self.current_state = FsmState::Code;
            BYTE_NOT_COUNTED
        }
    }

    // Just saw '/'
    fn process_slash(&mut self, b: u8) -> u64 {
        if b == b'*' {
            // start of block comment
            self.current_state = FsmState::Block;
            BYTE_NOT_COUNTED
        } else {
            // false alarm
            self.current_state = FsmState::Code;
            BYTE_NOT_COUNTED
        }
    }

    // Inside block comment
    fn process_comment(&mut self, b: u8) -> u64 {
        if b == b'*' {
            // maybe closing next
            self.current_state = FsmState::BlockStar;
            BYTE_NOT_COUNTED
        } else {
            // regular byte in body
            self.current_state = FsmState::Block;
            ONE_BYTE_COUNTED
        }
    }

    // Inside block, previous byte was '*'
    fn process_star(&mut self, b: u8) -> u64 {
        if b == b'/' {
            // end of block (delimiters not counted)
            self.current_state = FsmState::Code;
            BYTE_NOT_COUNTED
        } else if b == b'*' {
            // consecutive '*' is still body
            self.current_state = FsmState::BlockStar;
            ONE_BYTE_COUNTED
        } else {
            // Otherwise: previous '*' was content (+1) AND current byte (+1)
            self.current_state = FsmState::Block;
            TWO_BYTES_COUNTED
        }
    }

    fn current_state(&self) -> &FsmState {
        &self.current_state
    }
}

fn main() {
    let mut nb_bytes: u64 = 0;

    let mut my_fsm = Fsm::new();
    println!("Initial state: {:?}", my_fsm.current_state());

    let path = std::env::args().nth(1).expect("Provide the name of a c file.");
    let data = fs::read(&path).expect("Can't read the file.");

    for &current_byte in &data {
        nb_bytes += my_fsm.process_byte(current_byte);
    }

    println!("{nb_bytes}");
}
