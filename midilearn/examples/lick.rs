//! Run the learning code with a single lick.

extern crate midilearn;

use midilearn::{MidiLearn, Problem, User};

fn main() {
    let prob = Problem::new("Byard 1: C",
                            r#"{
                                "type": "lick",
                                "notes": [ [60],[62],[64],[67],[64,65],[67],[63,64],[60],[57],[60] ]
                            }"#);
    let prob_next = Problem::new("shell: Am7-D7-GΔ (1/7-3)",
        r#"{
            "type": "voicing",
            "chords": [[45,55,60],[38,54,60],[43,54,59]]
        }"#);

    MidiLearn::with_new(|learn: &mut MidiLearn| {
        println!("Status: {:?}", learn.single(&prob, Some(&prob_next)));
    }).unwrap();
}
