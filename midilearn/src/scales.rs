//! A midi sequence describes an expected sequence to be played.

use editdistancewf;
use Result;
use note::Note;

/// A vector of notes that are expected to be played linearly.  The notes can be played legato.
/// The inner vector holds notes that should be played simultaneously, such as with both hands.
#[derive(Debug)]
pub struct ScaleSeq(Vec<Vec<Note>>);

impl ScaleSeq {
    /// Generate the sequence for the described scale.
    pub fn from_scale(scale: &Scale) -> Result<ScaleSeq> {
        let base = Note::from_str(&scale.base)?;
        // println!("base: {:?}", base);

        // Build up the notes based on the interval.
        let mut first = base;
        let mut last = base;
        let mut notes = vec![base];
        for _ in 0 .. scale.octaves {
            for ch in scale.intervals.chars() {
                let next = match ch {
                    'H' => 1,
                    'W' => 2,
                    'm' => 3,
                    'M' => 4,
                    '4' => 5,
                    _ => return Err(format!("Invalid interval: {:?}", scale.intervals).into()),
                };
                let next = last + next;
                notes.push(next);
                last = next;
            }
            if first + 12 != last {
                return Err(format!("Scale is not exactly one octave: {:?}", scale.intervals).into());
            }
            first = last;
        }

        // Generate the appropriate scale type.
        match scale.style.as_str() {
            "updown" => {
                // Reflect the scale back on itself, except eliminate the duplicate note.
                let mut other = notes.clone();
                other.reverse();
                notes.extend(other.iter().skip(1));
            }
            style => return Err(format!("Unknown scale style: {:?}", style).into()),
        }

        // Build the note vectors out of the notes.
        let notes: Vec<_> = notes.iter().map(|&n| {
            let mut each = vec![n];
            for i in 1..scale.hands {
                each.push(n + (12 * i) as i8);
            }
            each
        }).collect();

        Ok(ScaleSeq(notes))
    }

    /// Scales can be played in a different octave than requested.  Compare the first note the user
    /// played with the note given.  If they are off by some number of octaves, adjust all of the
    /// notes in the ScaleSeq.  Returns true if the first note matches and a possible adjustment
    /// has been made.  Return false if there are either no notes, or the first not is not the
    /// right note.
    pub fn adjust_octave(&mut self, played: &[Vec<Note>]) -> bool {
        let my_first = match self.0.first() {
            None => return false,
            Some(v) => {
                match v.first() {
                    None => return false,
                    Some(&v) => v,
                }
            }
        };
        let played_first = match played.first() {
            None => return false,
            Some(v) => {
                match v.first() {
                    None => return false,
                    Some(&v) => v,
                }
            }
        };

        if my_first == played_first {
            return true;
        }

        let shift = played_first.0 as i32 - my_first.0 as i32;
        if shift % 12 == 0 {
            for me in &mut self.0 {
                for n in me.iter_mut() {
                    *n = *n + shift as i8;
                }
            }
            true
        } else {
            false
        }
    }

    /// Determine how different what the user played is from what is given.
    pub fn differences(&self, played: &[Vec<Note>]) -> usize {
        // println!("Compare: {:?} and\n         {:?}", self.0, played);
        editdistancewf::distance(self.0.iter(), played.iter())
    }
}

/// A structure mapping to the json data that is the input for a scale.
#[derive(Debug, Deserialize)]
pub struct Scale {
    pub base: String,
    pub intervals: String,
    pub hands: u32,
    pub octaves: u32,
    pub style: String,
}
