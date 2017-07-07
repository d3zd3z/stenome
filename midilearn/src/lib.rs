//! Using MIDI, verify that the user is able to play a given exercise correctly.  The question
//! should be enough of a description to uniquely describe what should be played.  The answer
//! should be a parseable description of what needs to be played.
//!
//! The answer should be a JSON description of what to play, of one of the following types:
//!
//! { "type": "scale", "base": "C♯", "intervals": "WWHWWWH",
//!   "hands": 1, "octaves": 2, "style": "updown" }
//!
//! This describes a scale.  The base is the first note of the scale.  Sharps and flags can either
//! be "#" and "b" or Unicode characters "♯", and "♭".  The intervals describe the intervals
//! between each note.  Valid characters are 'H' for a half step, 'W' for a whole step, 'm' for a
//! minor third, 'M' for a major third, and '4' for a fourth.  It will be checked that the
//! intervals cover a full octabe.  Hands describes how many hands should be playing the exercise,
//! and octaves are how many octaves to play.  The currently only supported style is "updown",
//! which requires the scale be played from base to base, for the given number of octaves and then
//! back down.  Scales can be played starting in any octave.
//!
//! { "type": "voicing", "chords": [
//!    { "base": "D", "notes": [ "3", "7", "9", "5" },
//!    { "base": "G", "notes": [ "7", "3", "#9", "#5" },
//!    { "base": "C", "notes": [ "3", "7", "9", "5", "11" } ] }
//!
//! This describes a voicing exercise, which is a series of chords to be played.  Each chord has a
//! base, which may or may not actually be played.  The notes in the chord are described in musical
//! notation (again with text or Unicode sharp/flat indicators).  The chord will be built using the
//! next note of the given function.  The octave will be chosen so that the lowest non-root note
//! will be between F#-F surrounding middle C.

#![deny(missing_docs)]

extern crate editdistancewf;
extern crate portmidi;
extern crate timelearn;
#[macro_use] extern crate lazy_static;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

use portmidi::{PortMidi, InputPort};
use serde_json::{Value};
use std::error;
use std::io::{self, Write};
use std::result;
use std::thread;
use std::time::Duration;
pub use timelearn::{Problem, Status, User};

mod note;
use note::Note;

mod sequence;
use sequence::{Scale, ScaleSeq};

/// Just box the errors for now.  TODO: Use a proper error type.
pub type Result<T> = result::Result<T, Box<error::Error + Send + Sync>>;

/// A learner based on playing the exercises on a MIDI device.
pub struct MidiLearn {
    input: InputPort,
}

impl MidiLearn {
    /// Construct a new MIDI learning, using the default MIDI port.  Invokes `f` with the
    /// MidiLearn.
    pub fn with_new<F, R>(f: F) -> Result<R>
        where F: FnOnce(&mut MidiLearn) -> R
    {
        let pm = PortMidi::new()?;
        let input = pm.default_input_port(4096)?;
        let mut learn = MidiLearn {
            input: input,
        };
        let result = f(&mut learn);
        Ok(result)
    }

}

impl User for MidiLearn {
    /// Practice a single exercise.  Waits for the user to play the exercise, and then returns a
    /// status indicating how well it was played.
    fn single(&mut self, word: &Problem) -> Result<Status> {
        println!("Play: {}", word.question);

        let json: Value = serde_json::from_str(&word.answer)?;
        // println!("Json: {:?}, {:?}", json, json["typr"]);
        if is_type(&json, "scale") {
            let scale: Scale = serde_json::from_value(json)?;

            let mut seq = ScaleSeq::from_scale(&scale)?;
            // println!("scale: {:?}", seq);

            self.drain()?;
            let user = self.record_scale()?;
            // println!("Played: {:?}", user);
            if seq.adjust_octave(&user) {
                // println!("First note good: {:?}", seq);
                let diff_count = seq.differences(&user);
                println!("There are {} differences", diff_count);
                if diff_count <= 3 {
                    Ok(Status::Continue(4 - diff_count as u8))
                } else {
                    Ok(Status::Continue(1))
                }
            } else {
                println!("First note mismatch, stopping");
                Ok(Status::Stopped)
            }

        } else {
            return Err(format!("Invalid type: {:?}", json["type"].as_str()).into());
        }
    }
}

impl MidiLearn {
    /// Drain any queued midi events.
    fn drain(&mut self) -> Result<()> {
        loop {
            match self.input.read()? {
                Some(_) => (),
                None => break,
            }
        }
        Ok(())
    }

    /// Record the user playing a scale.  Consider the scale done after a small pause of no
    /// playing.
    fn record_scale(&mut self) -> Result<Vec<Note>> {
        let mut notes = vec![];
        let mut idle_count = 0;
        loop {
            match self.input.read()? {
                Some(ev) => {
                    // We only care about note down events here.
                    if ev.message.status & 0xf0 == 0x80 {
                        // println!("Got notes: {:?}", ev);
                        notes.push(Note(ev.message.data1));
                        idle_count = 0;
                    }
                },
                None => {
                    idle_count += 1;
                    if notes.is_empty() {
                        idle_count = 0;
                    }
                    if idle_count >= 3 {
                        println!("Timing out");
                        break;
                    }
                    thread::sleep(Duration::new(1, 0));
                }
            }
        }
        Ok(notes)
    }
}

impl Write for MidiLearn {
    fn flush(&mut self) -> io::Result<()> {
        io::stdout().flush()
    }

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        io::stdout().write(buf)
    }
}

// Determine if the json has a the given type.
fn is_type(json: &Value, t: &str) -> bool {
    json["type"] == json!(t)
}
