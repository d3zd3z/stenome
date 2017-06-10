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
use learn::Learn;
use term::Term;

pub type Result<T> = result::Result<T, Box<error::Error + Send + Sync>>;

mod stroke;
mod learn;
mod term;
pub mod legacy;

pub fn run() {
    let st = Store::open("words.db").unwrap();

    let term = Term::new().unwrap();

    let mut learn = Learn::new(st, term);
    learn.run();
    /*
    let words = learn.into_words();
    words.save("learning.json").unwrap();
    */
}
