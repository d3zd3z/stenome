// Learn Steno using SRA.

extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

#[cfg(test)]
extern crate tempdir;

use std::error;
use std::result;

pub use stroke::Stroke;
pub use words::Words;

pub type Result<T> = result::Result<T, Box<error::Error + Send + Sync>>;

mod stroke;
mod words;
