// Learn Steno using SRA.

extern crate rand;
extern crate rusqlite;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate termion;

extern crate timelearn;

#[cfg(feature = "midi")]
extern crate midilearn;

#[cfg(test)]
extern crate tempdir;

use std::error;
use std::result;

#[cfg(feature = "midi")]
use midilearn::MidiLearn;
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
        run_steno(st);
    } else if st.get_kind() == "simple" {
        run_simple(st);
    } else if st.get_kind() == "midi" {
        run_midi(st);
    } else {
        panic!("Unknown store type '{}'", st.get_kind());
    }
}

fn run_steno(st: Store) {
    let mut user = Steno::new().unwrap();
    let mut learn = Learn::new(st, &mut user);
    learn.run();
}

fn run_simple(st: Store) {
    let mut user = Simple::new().unwrap();
    let mut learn = Learn::new(st, &mut user);
    learn.run();
}

#[cfg(feature = "midi")]
fn run_midi(st: Store) {
    MidiLearn::with_new(|user: &mut MidiLearn| {
        let mut learn = Learn::new(st, user);
        learn.run();
    }).unwrap();
}

#[cfg(not(feature = "midi"))]
fn run_midi(_st: Store) {
    panic!("Program not built with midi support");
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
