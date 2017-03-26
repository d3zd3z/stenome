// Learn Steno using SRA.

use std::error;
use std::result;

pub use stroke::Stroke;

pub type Result<T> = result::Result<T, Box<error::Error + Send + Sync>>;

mod stroke;
