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
//!
//! For the notes, values can be chord positions.  It is also possible to have a note that is a
//! placeholder, as an empty string, which indicates that the following note should be moved up an
//! octave.
//!
//! { "type": "lick", "notes": [ [60],[62],[64],[67],[64,65],[67],[63,64],[60],[57],[60] ] }
//!
//! This describes a lick.  They differ from scales in that they are just given as midi note
//! values, like scales they can be played in a different octave.  The values are just the midi
//! notes to be played.

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

mod scales;
use scales::{Lick, Scale, ScaleSeq, Voicing};

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
    /// status indicating how well it was played.  Next can be set to the possibly next problem,
    /// which will be shown after the current one.
    fn single(&mut self, word: &Problem, next: Option<&Problem>) -> Result<Status> {
        let st1 = self.single_once(word, next)?;

        let mut stn = st1;
        loop {
            match stn {
                // If good, return the initial status.
                Status::Continue(4) => return Ok(st1),
                Status::Stopped => return Ok(Status::Stopped),
                _ => (),
            }
            println!("** Mistakes made, please play again **");
            stn = self.single_once(word, next)?;
        }
    }
}

impl MidiLearn {
    /// Ask the user once to play.
    fn single_once(&mut self, word: &Problem, next: Option<&Problem>) -> Result<Status> {
        println!("Play: {}", word.question);
        match next {
            None => (),
            Some(n) => println!("      {}", n.question),
        }

        let json: Value = serde_json::from_str(&word.answer)?;
        // println!("Json: {:?}, {:?}", json, json["typr"]);
        if is_type(&json, "scale") {
            let scale: Scale = serde_json::from_value(json)?;

            let mut seq = ScaleSeq::from_scale(&scale)?;
            // println!("scale: {:?}", seq);

            self.drain()?;
            let user = self.record_scale(6)?;
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
        } else if is_type(&json, "lick") {
            let lick: Lick = serde_json::from_value(json)?;

            let mut seq = ScaleSeq::from_lick(&lick)?;

            self.drain()?;
            let user = self.record_scale(6)?;
            // println!("Played: {:?}", user);
            if seq.adjust_octave(&user) {
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

        } else if is_type(&json, "voicing") {
            let chords: Voicing = serde_json::from_value(json)?;

            let seq = ScaleSeq::from_voicing(&chords)?;
            // println!("chords: {:?}", seq);
            self.drain()?;
            let user = self.record_scale(8)?;
            // println!("user: {:?}", user);
            if user[0].len() == 1 {
                println!("Single note, stopping");
                Ok(Status::Stopped)
            } else {
                let diff_count = seq.differences(&user);
                println!("There are {} differences", diff_count);
                if diff_count == 0 {
                    Ok(Status::Continue(4))
                } else {
                    Ok(Status::Continue(1))
                }
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
    /// playing.  The timeout, is the number of 250ms ticks to consider the recording done.
    fn record_scale(&mut self, timeout: usize) -> Result<Vec<Vec<Note>>> {
        let mut notes: Vec<Vec<Note>> = vec![];
        let mut idle_count = 0;
        let mut last_time = 0;
        loop {
            match self.input.read()? {
                Some(ev) => {
                    // We only care about note down events here.
                    if ev.message.status & 0xf0 == 0x90 {
                        if !notes.is_empty() && ev.timestamp - last_time < 80 {
                            notes.last_mut().unwrap().push(Note(ev.message.data1));
                            // println!("chord : {:?}", notes);
                        } else {
                            // Individual note.
                            notes.push(vec![Note(ev.message.data1)]);
                            // println!("single: {:?}", notes);
                        }
                        idle_count = 0;
                        last_time = ev.timestamp;
                    }
                },
                None => {
                    idle_count += 1;
                    if notes.is_empty() {
                        idle_count = 0;
                    }
                    if idle_count >= timeout {
                        println!("Timing out");
                        break;
                    }
                    thread::sleep(Duration::from_millis(250));
                }
            }
        }

        // Sort the chords, since the incoming order could differ.
        for ch in notes.iter_mut() {
            ch.sort();
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
