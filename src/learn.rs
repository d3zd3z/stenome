// Learning.

use steno::Steno;
use timelearn::{Problem, Store};
use humanize_time;

use std::io::Write;
use Status;

pub struct Learn {
    store: Store,
    user: Steno,
}

impl Learn {
    pub fn new(store: Store, user: Steno) -> Learn {
        Learn {
            store: store,
            user: user,
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

        writeln!(self.user,
                 "\r\nActive: {}, Later: {}, Unlearned: {}, Interval {}\r",
                 counts.active,
                 counts.later,
                 counts.unlearned,
                 humanize_time(word.get_interval()))
                .unwrap();
        let mut active = 0;
        let mut learned = 0;
        for b in &counts.buckets {
            writeln!(self.user,
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
        writeln!(self.user, "  active : {}\r", active).unwrap();
        writeln!(self.user, "  learned: {}\r\n\r", learned).unwrap();
        self.user.flush().unwrap();

        self.user.single(word)
    }
}

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
