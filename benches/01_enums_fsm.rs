// cargo add criterion

// [[bench]]
// name = "01_enums_fsm"
// harness = false

// cargo bench --bench 01_enums_fsm

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
    statistics: TextStats,
}

impl Fsm {
    fn new() -> Self {
        Self {
            current_state: FsmState::Whitespace,
            statistics: TextStats::default(),
        }
    }

    fn process_char(&mut self, c: char) {
        match self.current_state {
            FsmState::Whitespace => {
                if c.is_alphabetic() {
                    self.current_state = FsmState::InWord;
                    self.statistics.word_count += 1;
                } else if c.is_numeric() {
                    self.current_state = FsmState::InNumber;
                    self.statistics.number_count += 1;
                } else if c == '\n' {
                    self.statistics.line_count += 1;
                }
            }
            FsmState::InWord => {
                if !c.is_alphabetic() {
                    self.current_state = FsmState::Whitespace;
                    if c == '\n' {
                        self.statistics.line_count += 1;
                    }
                }
            }
            FsmState::InNumber => {
                if !c.is_numeric() {
                    self.current_state = FsmState::Whitespace;
                    if c == '\n' {
                        self.statistics.line_count += 1;
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

fn load_file_contents() -> String {
    let path = Path::new("./benches/book.txt");
    let file = File::open(path).expect("Failed to open book.txt");
    let reader = BufReader::new(file);

    let mut contents = String::new();
    for line in reader.lines() {
        contents.push_str(&line.unwrap());
        contents.push('\n');
    }

    contents
}

fn benchmark_enum_fsm(c: &mut Criterion) {
    let text = load_file_contents();

    // --- One-time sanity check: NOT measured ---
    // Do a single parse and print the stats so you can verify values.
    let mut check = Fsm::new();
    check.process_text(&text);
    println!(
        "Sanity stats -> words: {}, lines: {}, numbers: {}",
        check.statistics.word_count, check.statistics.line_count, check.statistics.number_count
    );

    // --- Actual benchmark: measured ---
    c.bench_function("enum_fsm_parsing", |b| {
        b.iter(|| {
            // Rebuild the parser each iteration so we measure a full parse
            let mut parser = Fsm::new();
            parser.process_text(black_box(&text));
            // Return stats to keep work observable; black_box to defeat DCE further
            black_box(parser.statistics)
        })
    });
}
criterion_group!(benches, benchmark_enum_fsm);
criterion_main!(benches);
