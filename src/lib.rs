// Learn Steno using SRA.

extern crate rand;
extern crate rusqlite;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate termion;

extern crate timelearn;

#[cfg(test)]
extern crate tempdir;

use std::error;
use std::io::Write;
use std::result;

pub use stroke::Stroke;
// pub use words::{Counts, LearnWord, Words, Store};
use timelearn::Store;
pub use timelearn::Problem;
use learn::Learn;
use steno::Steno;

pub type Result<T> = result::Result<T, Box<error::Error + Send + Sync>>;

mod stroke;
mod learn;
mod steno;
pub mod legacy;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Status {
    Continue(u8),
    Stopped,
}

/// A User is something that can be asked to solve a single problem.  It implements `Write` which
/// is used to prompt and present information.  The method `single` is used to ask a single
/// question, and get status back from it.
pub trait User: Write {
    fn single(&mut self, word: &Problem) -> Result<Status>;
}

pub fn run() {
    let st = Store::open("words.db").unwrap();

    if st.get_kind() != "steno" {
        panic!("Store is not a \"steno\" store ({:?})", st.get_kind());
    }

    let mut user = Steno::new().unwrap();

    let mut learn = Learn::new(st, &mut user);
    learn.run();
    /*
    let words = learn.into_words();
    words.save("learning.json").unwrap();
    */
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
