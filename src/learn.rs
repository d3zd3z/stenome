// Learning.

use term::Term;
use stroke::Stroke;
use timelearn::{Problem, Store};

use std::io::Write;
use termion::color;

pub struct Learn {
    store: Store,
    term: Term,
}

impl Learn {
    pub fn new(store: Store, term: Term) -> Learn {
        Learn {
            store: store,
            term: term,
        }
    }

    pub fn run(&mut self) {
        loop {
            // TODO: Check for things that have expired we need to learn.

            let mut word = match self.store.get_next().unwrap() {
                None => {
                    match self.store.get_new().unwrap() {
                        None => {
                            println!("No more words to learn\r");
                            return;
                        }
                        Some(word) => word,
                    }
                }
                Some(word) => word,
            };

            let status = self.single(&mut word);

            match status {
                Status::Stopped => break,
                Status::Continue(factor) => self.store.update(word, factor).unwrap(),
            }
        }
    }

    // Learn a single word, updating its timing information based on how well it was learned.
    fn single(&mut self, word: &Problem) -> Status {
        let counts = self.store.get_counts().unwrap();

        writeln!(self.term,
                 "\r\nActive: {}, Later: {}, Unlearned: {}, Interval {}\r",
                 counts.active,
                 counts.later,
                 counts.unlearned,
                 humanize_time(word.interval))
                .unwrap();
        let mut active = 0;
        let mut learned = 0;
        for b in &counts.buckets {
            writeln!(self.term,
                     "  {:-4}: {:4} {}\r",
                     b.name,
                     b.count,
                     stars(65, b.count, counts.active + counts.later))
                    .unwrap();
            // TODO: This shouldn't be done by string matching, it is fragile.
            if b.name == "day" || b.name == "mon" {
                learned += b.count;
            } else {
                active += b.count;
            }
        }
        writeln!(self.term, "  active : {}\r", active).unwrap();
        writeln!(self.term, "  learned: {}\r\n\r", learned).unwrap();
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

static UNITS: &'static [UnitEntry] = &[UnitEntry {
     name: "seconds",
     div: 60.0,
 },
 UnitEntry {
     name: "minutes",
     div: 60.0,
 },
 UnitEntry {
     name: "hours",
     div: 24.0,
 },
 UnitEntry {
     name: "days",
     div: 365.0,
 },
 UnitEntry {
     name: "months",
     div: 12.0,
 },
 UnitEntry {
     name: "years",
     div: 1.0e6,
 }];

// Print a line of stars resembling a histogram bar.  `len` is the number of stars to use, a is the
// number in question, and total is the expected total.
fn stars(len: usize, value: usize, total: usize) -> String {
    let mut buf = String::new();
    buf.push('|');
    let thresh = value as f64 / total as f64 * len as f64;
    for i in 0..len {
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
    word: &'w Problem,
    strokes: Vec<Stroke>,
    user: Vec<Stroke>,
    errors: u32,
}

impl<'t, 'w> Single<'t, 'w> {
    fn new<'tt, 'ww>(term: &'tt mut Term, word: &'ww Problem) -> Single<'tt, 'ww> {
        Single {
            term: term,
            word: word,
            strokes: Stroke::parse_strokes(&word.answer).unwrap(),
            user: vec![],
            errors: 0,
        }
    }

    fn prompt(&mut self) {
        write!(self.term,
               "\r\x1b[J{:20}: {}{}",
               self.word.question,
               if self.strokes == self.user {
                   if self.errors == 0 { '✓' } else { '✗' }
               } else {
                   ' '
               },
               slashed(&self.user, &self.strokes))
                .unwrap();
        if self.errors > 0 {
            write!(self.term, "  ({})", slashed(&self.strokes, &self.strokes)).unwrap();
        }
        self.term.flush().unwrap();
    }

    fn run(&mut self) -> Status {
        let result;
        loop {
            self.prompt();
            if self.strokes == self.user {
                result = Status::Continue(if self.errors > 0 { 1 } else { 4 });
                break;
            }

            let stroke = match self.term.read_stroke().unwrap() {
                None => {
                    result = Status::Stopped;
                    break;
                }
                Some(st) => st,
            };
            if stroke.is_star() {
                self.user.pop();
            } else {
                self.user.push(stroke);
                let pos = self.user.len();
                if pos > self.strokes.len() || self.user[pos - 1] != self.strokes[pos - 1] {
                    self.errors += 1;
                }
            }
        }

        match result {
            Status::Continue(_) => {
                writeln!(self.term,
                         "\r\nNew interval {}\r",
                         humanize_time(self.word.interval))
                        .unwrap();
            }
            Status::Stopped => writeln!(self.term, "\r").unwrap(),
        }
        self.term.flush().unwrap();
        result
    }
}

// Generate a slash separated version of the given stroke list.
fn slashed(strokes: &[Stroke], expected: &[Stroke]) -> String {
    let mut buf = vec![];
    let mut first = true;

    for (i, st) in strokes.iter().enumerate() {
        if !first {
            buf.push(b'/');
        }
        first = false;

        let correct = expected.get(i) == Some(st);
        if !correct {
            write!(&mut buf,
                   "{}{}",
                   color::Bg(color::LightRed),
                   color::Fg(color::Black))
                    .unwrap();
        }
        write!(&mut buf, "{}", st).unwrap();
        if !correct {
            write!(&mut buf,
                   "{}{}",
                   color::Bg(color::Reset),
                   color::Fg(color::Reset))
                    .unwrap();
        }
    }
    String::from_utf8(buf).unwrap()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Status {
    Continue(u8),
    Stopped,
}
