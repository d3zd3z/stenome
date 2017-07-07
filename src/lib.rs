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
use std::result;

pub use stroke::Stroke;
// pub use words::{Counts, LearnWord, Words, Store};
use timelearn::Store;
pub use timelearn::{Problem, Status, User};
use learn::Learn;
use steno::Steno;
use simple::Simple;

pub type Result<T> = result::Result<T, Box<error::Error + Send + Sync>>;

mod stroke;
mod learn;
mod simple;
mod steno;
pub mod legacy;

pub fn run(path: &str) {
    let st = Store::open(path).unwrap();

    if st.get_kind() == "steno" {
        let mut user = Steno::new().unwrap();
        let mut learn = Learn::new(st, &mut user);
        learn.run();
    } else if st.get_kind() == "simple" {
        let mut user = Simple::new().unwrap();
        let mut learn = Learn::new(st, &mut user);
        learn.run();
    } else {
        panic!("Unknown store type '{}'", st.get_kind());
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
