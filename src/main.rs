extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

// Stenome expects Plover to do the decoding of the steno keyboard.  To make this work, you should
// either have an empty user dictionary, or add an empty dictionary to the list.  Then, remove all
// of the other dictionaries.  This will cause everything to be untranslatable.  Also, change the
// output settings so that the space is sent after the stroke, rather than before.  This allows us
// to decode the raw steno strokes as they are sent.

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut chars = String::new();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Esc => break,
            Key::Char(' ') => {
                if !chars.is_empty() {
                    println!("Stroke: {:?}\r", chars);
                    chars.clear();
                }
            }
            Key::Char(ch) => {
                chars.push(ch);
            }
            other => {
                println!("Unknown: {:?}\r", other);
            }
        }
        stdout.flush().unwrap();
    }
}
