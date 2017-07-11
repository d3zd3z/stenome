//! Run the learning code with a single scale.

extern crate midilearn;

use midilearn::{MidiLearn, Problem, User};

fn main() {
    let prob = Problem::new("shell: Dm7-G7-CΔ (1/3-7)",
                            r#"{
                                "type": "voicing",
                                "chords": [[50,60,65],[43,59,65],[48,59,64]]
                            }"#);
    println!("guide: Dm7-G7-CΔ (1/3-7)");

    MidiLearn::with_new(|learn: &mut MidiLearn| {
        println!("Status: {:?}", learn.single(&prob));
    }).unwrap();
}
