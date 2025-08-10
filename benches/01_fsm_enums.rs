// cargo add criterion

// [[bench]]
// name = "01_fsm_enums"
// harness = false

// cargo bench --bench 01_fsm_enums

use criterion::{Criterion, criterion_group, criterion_main};
use std::fs::File;
use std::hint::black_box;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone, Copy)]
enum FsmState {
    Whitespace,
    InWord,
    InNumber,
}

#[derive(Default)]
struct TextStats {
    word_count: usize,
    line_count: usize,
    number_count: usize,
}

struct Fsm {
    current_state: FsmState,
    stats: TextStats,
    current_line: usize,
}

impl Fsm {
    fn new() -> Self {
        Self {
            current_state: FsmState::Whitespace,
            stats: TextStats::default(),
            current_line: 1,
        }
    }

    fn process_char(&mut self, c: char) {
        match self.current_state {
            FsmState::Whitespace => {
                if c.is_alphabetic() {
                    self.current_state = FsmState::InWord;
                    self.stats.word_count += 1;
                } else if c.is_numeric() {
                    self.current_state = FsmState::InNumber;
                    self.stats.number_count += 1;
                } else if c == '\n' {
                    self.stats.line_count += 1;
                    self.current_line += 1;
                }
            }
            FsmState::InWord => {
                if !c.is_alphabetic() {
                    self.current_state = FsmState::Whitespace;
                    if c == '\n' {
                        self.stats.line_count += 1;
                        self.current_line += 1;
                    }
                }
            }
            FsmState::InNumber => {
                if !c.is_numeric() {
                    self.current_state = FsmState::Whitespace;
                    if c == '\n' {
                        self.stats.line_count += 1;
                        self.current_line += 1;
                    }
                }
            }
        }
    }

    fn process_text(&mut self, text: &str) {
        for c in text.chars() {
            self.process_char(c);
        }
    }
}

fn benchmark_enum_fsm(c: &mut Criterion) {
    let path = Path::new("./benches/book.txt");
    let file = File::open(path).expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut full_text = String::new();
    for line in reader.lines() {
        full_text.push_str(&line.unwrap());
        full_text.push('\n');
    }

    c.bench_function("enum_fsm_text_parsing", |b| {
        b.iter(|| {
            let mut parser = Fsm::new();
            parser.process_text(black_box(&full_text));
            parser.stats
        })
    });
}

criterion_group!(benches, benchmark_enum_fsm);
criterion_main!(benches);
