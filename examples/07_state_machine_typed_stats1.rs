// cargo add criterion

// [[bench]]
// name = "01_typed_fsm"
// harness = false

// cargo run --example 03_typed_fsm
// cargo bench --bench 03_typed_fsm

// use criterion::{Criterion, criterion_group, criterion_main};
// use std::hint::black_box;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::marker::PhantomData;
use std::path::Path;

// --- Typestate markers (zero-sized types)
struct Whitespace;
struct InWord;
struct InNumber;

// --- Aggregated stats
#[derive(Default, Debug, Clone)]
struct TextStats {
    word_count: usize,
    line_count: usize,
    number_count: usize,
}

// --- Generic FSM carrying stats; the state is encoded by the type parameter
struct Fsm<State> {
    stats: TextStats,
    _state: PhantomData<State>,
}

impl Fsm<Whitespace> {
    fn new() -> Self {
        Self {
            stats: TextStats::default(),
            _state: PhantomData,
        }
    }

    /// Decide next state from Whitespace based on the current char.
    /// We return a Machine (sum type) so the caller can keep a single variable.
    fn process_char(&mut self, c: char) -> Machine {
        // Count newlines regardless of the next state
        if c == '\n' {
            self.stats.line_count += 1;
        }

        if c.is_alphabetic() {
            // TODO is_ascii_alphabetic() + is_ascii_digit()
            // First letter of a word
            self.stats.word_count += 1;
            Machine::Word(Fsm {
                stats: self.stats.clone(), // TODO avoid cloning
                _state: PhantomData,
            })
        } else if c.is_ascii_digit() {
            // First digit of a number
            self.stats.number_count += 1;
            Machine::Number(Fsm {
                stats: self.stats.clone(), // TODO avoid cloning
                _state: PhantomData,
            })
        } else {
            // Stay in Whitespace
            Machine::White(Fsm {
                stats: self.stats.clone(), // TODO avoid cloning
                _state: PhantomData,
            })
        }
    }
}

impl Fsm<InWord> {
    /// Words are maximal runs of alphabetic chars.
    /// - Letter => stay in word
    /// - Digit  => start a number token
    /// - Other  => go to whitespace
    fn process_char(&mut self, c: char) -> Machine {
        if c == '\n' {
            // Newline is also a word boundary
            self.stats.line_count += 1;
            return Machine::White(Fsm {
                stats: self.stats.clone(), // TODO avoid cloning
                _state: PhantomData,
            });
        }

        if c.is_alphabetic() {
            // Still in the same word
            Machine::Word(Fsm {
                stats: self.stats.clone(), // TODO avoid cloning
                _state: PhantomData,
            })
        } else if c.is_ascii_digit() {
            // Word -> Number boundary: count a new number token
            self.stats.number_count += 1;
            Machine::Number(Fsm {
                stats: self.stats.clone(), // TODO avoid cloning
                _state: PhantomData,
            })
        } else {
            // Any non-alnum boundary => whitespace
            Machine::White(Fsm {
                stats: self.stats.clone(), // TODO avoid cloning
                _state: PhantomData,
            })
        }
    }
}

impl Fsm<InNumber> {
    /// Numbers are maximal runs of ASCII digits.
    /// - Digit  => stay in number
    /// - Letter => start a word token
    /// - Other  => go to whitespace
    fn process_char(&mut self, c: char) -> Machine {
        if c == '\n' {
            // Newline is also a number boundary
            self.stats.line_count += 1;
            return Machine::White(Fsm {
                stats: self.stats.clone(), // TODO avoid cloning
                _state: PhantomData,
            });
        }

        if c.is_ascii_digit() {
            // Still in the same number
            Machine::Number(Fsm {
                stats: self.stats.clone(), // TODO avoid cloning
                _state: PhantomData,
            })
        } else if c.is_alphabetic() {
            // Number -> Word boundary: count a new word token
            self.stats.word_count += 1;
            Machine::Word(Fsm {
                stats: self.stats.clone(), // TODO avoid cloning
                _state: PhantomData,
            })
        } else {
            // Any non-alnum boundary => whitespace
            Machine::White(Fsm {
                stats: self.stats.clone(), // TODO avoid cloning
                _state: PhantomData,
            })
        }
    }
}

// --- Sum type wrapper that lets us expose a single `process_char` API
enum Machine {
    White(Fsm<Whitespace>),
    Word(Fsm<InWord>),
    Number(Fsm<InNumber>),
}

impl Machine {
    fn new() -> Self {
        Machine::White(Fsm::new())
    }

    /// Process a character and update self in-place.
    /// This keeps ownership simple for the caller.
    fn process_char(&mut self, c: char) {
        // Pattern-match the current variant and delegate to the state's logic
        let next = match self {
            Machine::White(f) => f.process_char(c),
            Machine::Word(f) => f.process_char(c),
            Machine::Number(f) => f.process_char(c),
        };
        *self = next;
    }

    /// Borrow stats (identical regardless of the current state)
    fn stats(&self) -> &TextStats {
        match self {
            Machine::White(f) => &f.stats,
            Machine::Word(f) => &f.stats,
            Machine::Number(f) => &f.stats,
        }
    }
}

fn process_text(text: &str) -> TextStats {
    // Drive the FSM through the enum wrapper
    let mut m = Machine::new();
    for c in text.chars() {
        m.process_char(c);
    }
    m.stats().clone() // TODO avoid cloning
}

fn load_file_contents() -> String {
    let path = Path::new("./benches/book.txt");
    let file = File::open(path).expect("Failed to open book.txt");
    let reader = BufReader::new(file);

    let mut contents = String::new();
    for line in reader.lines() {
        // NOTE: This preserves original newlines so line_count works
        contents.push_str(&line.expect("I/O error while reading line"));
        contents.push('\n');
    }

    contents
}

fn main() {
    let text = load_file_contents();
    let stats = process_text(&text);
    println!("{:?}", stats);
}

// fn benchmark_typed_fsm(c: &mut Criterion) {
//     let text = load_file_contents();

//     // --- One-time sanity check: NOT measured ---
//     // Do a single parse and print the stats so you can verify values.
//     let stats = process_text(&text);
//     println!("Sanity stats -> words: {}, lines: {}, numbers: {}", stats.word_count, stats.line_count, stats.number_count);

//     // --- Actual benchmark: measured ---
//     c.bench_function("typed_fsm_parsing", |b| {
//         b.iter(|| {
//             let stats = process_text(black_box(&text));
//             // Return stats to keep work observable; black_box to defeat DCE further
//             black_box(stats)
//         })
//     });
// }

// criterion_group!(benches, benchmark_typed_fsm);
// criterion_main!(benches);
