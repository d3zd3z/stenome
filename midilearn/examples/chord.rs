//! Run the learning code with a single scale.

extern crate midilearn;

use midilearn::{MidiLearn, Problem, User};

fn main() {
    let prob = Problem::new("shell: Dm7-G7-CΔ (1/7-3)",
                            r#"{
                                "type": "voicing",
                                "chords": [[50,60,65],[43,59,65],[48,59,64]]
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
