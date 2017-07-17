//! Run the learning code with a two-hand scale.

extern crate midilearn;

use midilearn::{MidiLearn, Problem, User};

fn main() {
    let prob = Problem::new("scale: C major",
                            r#"{
                                "type": "scale",
                                "base": "C",
                                "intervals": "WWHWWWH",
                                "hands": 2,
                                "octaves": 2,
                                "style": "updown"
                            }"#);

    MidiLearn::with_new(|learn: &mut MidiLearn| {
        println!("Status: {:?}", learn.single(&prob, None));
    }).unwrap();
}
