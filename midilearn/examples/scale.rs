//! Run the learning code with a single scale.

extern crate midilearn;

use midilearn::{MidiLearn, Problem, User};

fn main() {
    let prob = Problem::new("scale: C major",
                            r#"{
                                "type": "scale",
                                "base": "C",
                                "intervals": "WWHWWWH",
                                "hands": 1,
                                "octaves": 2,
                                "style": "updown"
                            }"#);
    println!("Learn a single word");

    MidiLearn::with_new(|learn: &mut MidiLearn| {
        println!("Status: {:?}", learn.single(&prob));
    }).unwrap();
}
