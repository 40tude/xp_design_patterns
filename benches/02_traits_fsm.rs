// cargo add criterion

// [[bench]]
// name = "02_traits_fsm"
// harness = false

// cargo bench --bench 02_traits_fsm

use criterion::{Criterion, criterion_group, criterion_main};
use std::fs::File;
use std::hint::black_box;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Default, Debug)]
pub struct TextStats {
    word_count: usize,
    line_count: usize,
    number_count: usize,
}

// #[derive(Debug)] // ! derive not allowed on traits, only on concrete type => supertrait
pub trait FsmState: std::fmt::Debug {
    fn process_char(self: Box<Self>, c: char, statistics: &mut TextStats) -> Box<dyn FsmState>;
}

// States
#[derive(Debug)]
struct WhitespaceState;
impl FsmState for WhitespaceState {
    fn process_char(self: Box<Self>, c: char, statistics: &mut TextStats) -> Box<dyn FsmState> {
        if c.is_alphabetic() {
            statistics.word_count += 1;
            Box::new(InWordState)
        } else if c.is_numeric() {
            statistics.number_count += 1;
            Box::new(InNumberState)
        } else if c == '\n' {
            statistics.line_count += 1;
            self
        } else {
            self
        }
    }
}

#[derive(Debug)]
struct InWordState;
impl FsmState for InWordState {
    fn process_char(self: Box<Self>, c: char, statistics: &mut TextStats) -> Box<dyn FsmState> {
        if !c.is_alphabetic() {
            if c == '\n' {
                statistics.line_count += 1;
            }
            Box::new(WhitespaceState)
        } else {
            self
        }
    }
}

#[derive(Debug)]
struct InNumberState;
impl FsmState for InNumberState {
    fn process_char(self: Box<Self>, c: char, statistics: &mut TextStats) -> Box<dyn FsmState> {
        if !c.is_numeric() {
            if c == '\n' {
                statistics.line_count += 1;
            }
            Box::new(WhitespaceState)
        } else {
            self
        }
    }
}

struct TraitParser {
    state: Option<Box<dyn FsmState>>,
    statistics: TextStats,
}

impl TraitParser {
    fn new() -> Self {
        Self {
            state: Some(Box::new(WhitespaceState)),
            statistics: TextStats::default(),
        }
    }

    fn process_text(&mut self, text: &str) {
        for c in text.chars() {
            // ! Does not compile : cannot move out of `self.state` which is behind a mutable reference
            // self.state.unwrap() => try to get the value in Option (Box<dyn FsmState>) an move it outside of self.state
            // While at the same time : &mut self.statistics => mutable borrow on self
            // self.state = Some(self.state.unwrap().process_char(c, &mut self.statistics));

            // self.states.take() act only on .states field. It takes it a replace it by None (self.statistics remains untouched)
            let current_state = self.state.take().unwrap();
            self.state = Some(current_state.process_char(c, &mut self.statistics));
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

fn trait_fsm_benchmark(c: &mut Criterion) {
    let text = load_file_contents();

    // --- One-time sanity check: NOT measured ---
    // Do a single parse and print the stats so you can verify values.
    let mut check = TraitParser::new();
    check.process_text(&text);
    println!(
        "Sanity stats -> words: {}, lines: {}, numbers: {}",
        check.statistics.word_count, check.statistics.line_count, check.statistics.number_count
    );

    // --- Actual benchmark: measured ---
    c.bench_function("enum_fsm_parsing", |b| {
        b.iter(|| {
            // Rebuild the parser each iteration so we measure a full parse
            let mut parser = TraitParser::new();
            parser.process_text(black_box(&text));
            // Return stats to keep work observable; black_box to defeat DCE further
            black_box(parser.statistics)
        })
    });
}

criterion_group!(benches, trait_fsm_benchmark);
criterion_main!(benches);
