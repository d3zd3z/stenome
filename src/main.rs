extern crate termion;
extern crate stenome;

use std::env;

use stenome::Result;
use stenome::legacy::Words;

// Stenome expects Plover to do the decoding of the steno keyboard.  To make this work, you should
// either have an empty user dictionary, or add an empty dictionary to the list.  Then, remove all
// of the other dictionaries.  This will cause everything to be untranslatable.  Also, change the
// output settings so that the space is sent after the stroke, rather than before.  This allows us
// to decode the raw steno strokes as they are sent.

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    if args == &["create"] {
        create().unwrap();
    } else if args.len() == 2 && args[0] == "run" {
        stenome::run(&args[1]);
    } else if args.len() == 1 && args[0] == "record" {
        stenome::record();
    } else {
        println!("Usage: {{create|run db}}");
    }
    // stenome::run();
}

/// Create a new database, by loading data from an existing json file.
fn create() -> Result<()> {
    let words = Words::load("learning.json").unwrap();
    words.create_db("words.db").unwrap();
    Ok(())
}
