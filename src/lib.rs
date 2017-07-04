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
use steno::Steno;

pub type Result<T> = result::Result<T, Box<error::Error + Send + Sync>>;

mod stroke;
mod learn;
mod steno;
pub mod legacy;

pub fn run() {
    let st = Store::open("words.db").unwrap();

    let user = Steno::new().unwrap();

    let mut learn = Learn::new(st, user);
    learn.run();
    /*
    let words = learn.into_words();
    words.save("learning.json").unwrap();
    */
}
