//! A midi sequence describes an expected sequence to be played.

use editdistancewf;
use Result;
use note::Note;

/// A vector of notes that are expected to be played linearly.  The notes can be played legato.
/// The inner vector holds notes that should be played simultaneously, such as with both hands.
#[derive(Debug)]
pub struct ScaleSeq(pub Vec<Vec<Note>>);

impl ScaleSeq {
    /// Generate the sequence for the described scale.
    pub fn from_scale(scale: &Scale) -> Result<ScaleSeq> {
        let base = Note::from_str(&scale.base)?;
        // println!("base: {:?}", base);

        // Build up the notes based on the interval.
        let mut first = base;
        let mut last = base;
        let mut notes = vec![base];
        // For the simple exercise, just use the given octavies,  For the arp-type, add an octave
        // at each end.
        let octaves = if scale.style == "3up" || scale.style == "3upr" {
            scale.octaves + 2
        } else {
            scale.octaves
        };
        for _ in 0 .. octaves {
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

        fn build_pattern(orig: &[Note], octaves: u32, which: &[usize]) -> Vec<Note> {
            let per_octave = (orig.len() - 1) / octaves as usize;
            let mut notes = vec![];
            for i in per_octave .. 2 * per_octave {
                for off in which {
                    notes.push(orig[i + *off]);
                }
            }
            for i in (per_octave - 1 .. 2 * per_octave + 1).rev() {
                for off in which {
                    notes.push(orig[i + *off]);
                }
            }
            // And end with the starting note.
            notes.push(orig[per_octave]);

            notes
        }

        // Generate the appropriate scale type.
        match scale.style.as_str() {
            "updown" => {
                // Reflect the scale back on itself, except eliminate the duplicate note.
                let mut other = notes.clone();
                other.reverse();
                notes.extend(other.iter().skip(1));
            }
            "3up" => {
                // Go up and down in broken thirds going up.
                // We need to extend the scale by one note in each direction.
                notes = build_pattern(&notes, octaves, &[0, 2]);
            }
            "3upr" => {
                // Go up and down in broken thirds going up.
                // We need to extend the scale by one note in each direction.
                // With the pairs reversed.
                notes = build_pattern(&notes, octaves, &[2, 0]);
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

    /// Convert an input voicing into a sequence.  This is very straightforward, as the voicing
    /// exercises are generated before entering into the database.
    pub fn from_voicing(voicing: &Voicing) -> Result<ScaleSeq> {
        Ok(ScaleSeq(voicing.chords.iter().map(|chord| {
            chord.iter().map(|note| Note(*note)).collect()
        }).collect()))
    }

    /// Convert an input lick into a sequence.  This is also very straightforward.
    pub fn from_lick(lick: &Lick) -> Result<ScaleSeq> {
        Ok(ScaleSeq(lick.notes.iter().map(|chord| {
            chord.iter().map(|note| Note(*note)).collect()
        }).collect()))
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
        editdistancewf::distance(ChordNoteIter::new(&self.0),
            ChordNoteIter::new(played))
    }
}

// An iterator that spreads the notes out of a chord, and inserts a separator between them.  Since
// MIDI notes can only have the value 0-127, we can just use something like 128 as the chord
// marker.
struct ChordNoteIter<'a> {
    // The chords we're iterating over.  If the slice is empty, we're done.
    chords: &'a [Vec<Note>],
    // The position within the first chord.  If this is past the end of the chords, we're
    // generating the separator.
    pos: usize,
}

impl<'a> ChordNoteIter<'a> {
    fn new(chords: &[Vec<Note>]) -> ChordNoteIter {
        ChordNoteIter {
            chords: chords,
            pos: 0,
        }
    }
}

impl<'a> Iterator for ChordNoteIter<'a> {
    type Item = Note;

    fn next(&mut self) -> Option<Self::Item> {
        let chord = match self.chords.first() {
            None => return None,
            Some(ch) => ch,
        };

        if self.pos >= chord.len() {
            self.chords = &self.chords[1..];
            self.pos = 0;
            Some(Note(128))
        } else {
            let result = chord[self.pos];
            self.pos += 1;
            Some(result)
        }
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

/// A structure mapping to the json data that is the input for a voicing.
#[derive(Debug, Deserialize)]
pub struct Voicing {
    chords: Vec<Vec<u8>>,
}

/// A structure mapping to the json data that is the input for a lick.
#[derive(Debug, Deserialize)]
pub struct Lick {
    notes: Vec<Vec<u8>>,
}
