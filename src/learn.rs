// Learning.

use term::Term;
use stroke::Stroke;
use words::{LearnWord, Words};

use std::io::Write;

pub struct Learn {
    words: Words,
    term: Term,
}

impl Learn {
    pub fn new(words: Words, term: Term) -> Learn {
        Learn {
            words: words,
            term: term,
        }
    }

    // Consume the learner, returning the wordset contained in it.
    pub fn into_words(self) -> Words {
        self.words
    }

    pub fn run(&mut self) {
        loop {
            // TODO: Check for things that have expired we need to learn.

            let mut word = match self.words.get_learned() {
                None => {
                    match self.words.get_unlearned() {
                        None => {
                            println!("No more words to learn\r");
                            return;
                        },
                        Some(word) => word,
                    }
                }
                Some(word) => word,
            };

            let status = self.single(&mut word);

            // Return the word.
            self.words.push_word(word);

            if status == Status::Stopped {
                break;
            }
        }
    }

    // Learn a single word, updating its timing information based on how well it was learned.
    fn single(&mut self, word: &mut LearnWord) -> Status {
        let counts = self.words.get_counts();

        writeln!(self.term, "\r\nActive: {}, Later: {}, Unlearned: {}, Interval {}\r",
                 counts.active, counts.later, counts.unlearned,
                 humanize_time(word.interval)).unwrap();
        for b in &counts.buckets {
            writeln!(self.term, "  {:-4}: {:4} {}\r", b.name, b.count,
                     stars(65, b.count, counts.active + counts.later)).unwrap();
        }
        writeln!(self.term, "\r").unwrap();
        self.term.flush().unwrap();

        let mut state = Single::new(&mut self.term, word);
        state.run()
    }
}

// Format an interval (in seconds) in terms of nicer units.
fn humanize_time(interval: f64) -> String {
    let mut val = interval;
    for unit in UNITS {
        if val < unit.div {
            return format!("{:.1} {}", val, unit.name);
        }
        val /= unit.div;
    }
    // Out of bounds, return the unmodified time.
    format!("{:.1} seconds", interval)
}

struct UnitEntry {
    name: &'static str,
    div: f64,
}

static UNITS: &'static [UnitEntry] = &[
    UnitEntry{ name: "seconds", div: 60.0 },
    UnitEntry{ name: "minutes", div: 60.0 },
    UnitEntry{ name: "hours", div: 24.0 },
    UnitEntry{ name: "days", div: 365.0 },
    UnitEntry{ name: "months", div: 12.0 },
    UnitEntry{ name: "years", div: 1.0e6 },
];

// Print a line of stars resembling a histogram bar.  `len` is the number of stars to use, a is the
// number in question, and total is the expected total.
fn stars(len: usize, value: usize, total: usize) -> String {
    let mut buf = String::new();
    buf.push('|');
    let thresh = value as f64 / total as f64 * len as f64;
    for i in 0 .. len {
        if (i as f64) < thresh {
            buf.push('*');
        } else {
            buf.push(' ');
        };
    }
    buf.push('|');
    buf
}

struct Single<'t, 'w> {
    term: &'t mut Term,
    word: &'w mut LearnWord,
    user: Vec<Stroke>,
    errors: u32,
}

impl<'t, 'w> Single<'t, 'w> {
    fn new<'tt, 'ww>(term: &'tt mut Term, word: &'ww mut LearnWord) -> Single<'tt, 'ww> {
        Single {
            term: term,
            word: word,
            user: vec![],
            errors: 0,
        }
    }

    fn prompt(&mut self) {
        write!(self.term, "\r\x1b[J{:20}: {}{}",
               self.word.english,
               if self.word.strokes == self.user {
                   if self.errors == 0 { '✓' } else { '✗' }
               } else { ' ' },
               slashed(&self.user)).unwrap();
        if self.errors > 0 {
            write!(self.term, "  ({})", slashed(&self.word.strokes)).unwrap();
        }
        self.term.flush().unwrap();
    }

    fn run(&mut self) -> Status {
        let mut result = Status::Continue;
        loop {
            self.prompt();
            if self.word.strokes == self.user {
                break;
            }

            let stroke = match self.term.read_stroke().unwrap() {
                None => { result = Status::Stopped; break; }
                Some(st) => st,
            };
            if stroke.is_star() {
                self.user.pop();
            } else {
                self.user.push(stroke);
                let pos = self.user.len();
                if pos > self.word.strokes.len() ||
                    self.user[pos-1] != self.word.strokes[pos-1]
                {
                    self.errors += 1;
                }
            }
        }

        if result == Status::Continue {
            if self.errors > 0 {
                self.word.incorrect();
            } else {
                self.word.correct();
            }

            writeln!(self.term, "\r\nNew interval {}\r", humanize_time(self.word.interval)).unwrap();
        } else {
            writeln!(self.term, "\r").unwrap();
        }
        self.term.flush().unwrap();
        result
    }
}

// Generate a slash separated version of the given stroke list.
fn slashed(strokes: &[Stroke]) -> String {
    let mut buf = vec![];
    let mut first = true;

    for st in strokes {
        if !first {
            buf.push(b'/');
        }
        first = false;
        write!(&mut buf, "{}", st).unwrap();
    }
    String::from_utf8(buf).unwrap()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Status {
    Continue,
    Stopped,
}
