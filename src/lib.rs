// Learn Steno using SRA.

extern crate rand;
extern crate rusqlite;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate termion;

extern crate timelearn;

#[cfg(test)]
extern crate tempdir;

use std::error;
use std::result;

pub use stroke::Stroke;
pub use words::{Counts, LearnWord, Words, Store};
use learn::Learn;
use term::Term;

pub type Result<T> = result::Result<T, Box<error::Error + Send + Sync>>;

mod stroke;
mod words;
mod learn;
mod term;
pub mod legacy;

pub fn run() {
    // Try loading the words from a save file, if not present, create a new set of words.
    let words = match Words::load("learning.json") {
        Ok(w) => w,
        Err(_) => Words::new(),
    };

    let term = Term::new().unwrap();

    let mut learn = Learn::new(words, term);
    learn.run();
    let words = learn.into_words();
    words.save("learning.json").unwrap();
}
