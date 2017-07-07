//! Manipulations of MIDI notes.

use std::collections::HashMap;
use std::ops::Add;
use Result;

// The value of a single MIDI note.  A wrapper around the raw MIDI note value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Note(pub u8);

impl Note {
    /// Try to decode a textual node name.  Notes are a letter possibly followed by a sharp or flat
    /// sign (in ascii of Unicode).  The resulting note will be between middle C and the B above
    /// that.
    pub fn from_str(text: &str) -> Result<Note> {
        let mut base = None;
        let mut accidental = None;

        for ch in text.chars() {
            match NOTES.get(&ch) {
                Some(&new_base) => {
                    if !base.is_none() {
                        return Err(format!("Invalid note {:?}", text).into());
                    }
                    base = Some(new_base);
                    continue;
                }
                None => (),
            }

            match ACCIDENTALS.get(&ch) {
                Some(&new_accidental) => {
                    if !accidental.is_none() {
                        return Err(format!("Invalid note {:?}", text).into());
                    }
                    if base.is_none() {
                        return Err(format!("Invalid note {:?}", text).into());
                    }
                    accidental = Some(new_accidental);
                    continue;
                }
                None => (),
            }

            return Err(format!("Invalid note {:?}", text).into());
        }

        if base.is_none() {
            return Err(format!("Invalid note {:?}", text).into());
        }
        Ok(Note((base.unwrap() as i8 + accidental.unwrap_or(0)) as u8))
    }
}

impl Add<i8> for Note {
    type Output = Note;

    fn add(self, other: i8) -> Note {
        Note((self.0 as i8 + other) as u8)
    }
}

lazy_static! {
    static ref NOTES: HashMap<char, u8> = {
        let mut m = HashMap::new();
        m.insert('C', 60);
        m.insert('D', 62);
        m.insert('E', 64);
        m.insert('F', 65);
        m.insert('G', 67);
        m.insert('A', 69);
        m.insert('B', 71);
        m
    };

    static ref ACCIDENTALS: HashMap<char, i8> = {
        let mut m = HashMap::new();
        m.insert('#', 1);
        m.insert('♯', 1);
        m.insert('b', -1);
        m.insert('♭', -1);
        m
    };
}

#[cfg(test)]
mod test {
    use super::Note;

    #[test]
    fn test_note() {
        bad("C$");
        bad("C##");
        bad("C#b");
        bad("Cbb");
        bad("C♭♯");
        good("C", 60);
        good("Cb", 59);
        good("C#", 61);
        good("C♭", 59);
        good("C♯", 61);
        good("D", 62);
        good("Db", 61);
        good("D#", 63);
        good("D♭", 61);
        good("D♯", 63);
        good("E", 64);
        good("F", 65);
        good("G", 67);
        good("A", 69);
        good("B", 71);
        good("B#", 72);
    }

    fn good(text: &str, expect: u8) {
        match Note::from_str(text) {
            Ok(Note(value)) => assert_eq!(expect, value),
            Err(_) => panic!("Expected note to parse: {:?}", text),
        }
    }

    fn bad(text: &str) {
        match Note::from_str(text) {
            Ok(_) => panic!("Should not have parsed: {:?}", text),
            Err(_) => (),
        }
    }
}
