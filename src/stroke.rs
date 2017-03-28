//! A Steno Stroke represents a set of keys that are pressed together on the keyboard.

use ::Result;
use serde::{de, ser};
use std::fmt;
use std::result;

/// A Stroke is represented as a 32-bit unsigned integer, with a bit set for each key pressed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Stroke(pub u32);

impl Stroke {
    /// Parse a string representing one or more strokes (separated by slashes), returning the
    /// sequence if possible.
    pub fn parse_strokes(text: &str) -> Result<Vec<Stroke>> {
        let mut result = vec![];

        let mut full_iter = FULL_STENO.chars().enumerate();
        let mut pos = 0;
        let mut bits = 0u32;

        for ch in text.chars() {
            if ch == '/' {
                if bits == 0 {
                    return Err("Empty stroke".into());
                }
                result.push(Stroke(bits));
                full_iter = FULL_STENO.chars().enumerate();
                pos = 0;
                bits = 0;
                continue;
            }

            if ch == '-' {
                // Technically, this should be at position 7, but one of the lessons includes a
                // hyphen after a '*'.
                if pos >= 10 {
                    return Err(format!("Invalid '-' position in {:?}", text).into());
                }
                pos = 12;
                loop {
                    match full_iter.next() {
                        Some((bit, _)) if bit == 11 => break,
                        Some(_) => (),
                        None => panic!("Implementation error"),
                    }
                }
                continue;
            }

            // Loop through until we match.
            loop {
                match full_iter.next() {
                    Some((bit, full_ch)) => {
                        if ch == full_ch {
                            bits |= 1 << bit;
                            break;
                        }
                    }
                    None => return Err(format!("Invalid char in text: {:?} ({:?})", text, ch).into()),
                }
            }
        }
        if bits == 0 {
            return Err("Empty stroke".into());
        }
        result.push(Stroke(bits));

        Ok(result)
    }

    /// Parse a string containing a single stroke.
    pub fn parse_stroke(text: &str) -> Result<Stroke> {
        let mut itr = Self::parse_strokes(text)?.into_iter();

        let stroke = match itr.next() {
            None => return Err("No strokes in string".into()),
            Some(stroke) => stroke,
        };
        match itr.next() {
            None => (),
            Some(_) => return Err("Expecting only a single stroke".into()),
        }
        Ok(stroke)
    }

    /// Is this stroke just the '*'
    pub fn is_star(&self) -> bool {
        self.0 == STAR_STROKE
    }
}

impl fmt::Display for Stroke {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = self.0;

        let mut base = FULL_STENO;
        if value & NUM != 0 {
            if value & NUMS != 0 {
                base = NUM_STENO;
            } else {
                write!(f, "#")?;
            }
        }
        let need_hyphen = (value & MID == 0) && (value & RIGHT != 0);

        let mut bit = 1;
        for ch in base.chars() {
            if bit == FSTROKE && need_hyphen {
                write!(f, "-")?;
            }

            if value & bit != 0 {
                write!(f, "{}", ch)?;
            }
            bit <<= 1;
        }

        Ok(())
    }
}

// Serialize for Stroke.
impl ser::Serialize for Stroke {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: ser::Serializer
    {
        let text = format!("{}", self);
        serializer.serialize_str(&text)
    }
}

// Deserialize for Stroke.
impl de::Deserialize for Stroke {
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
        where D: de::Deserializer
    {
        deserializer.deserialize_str(StrokeVisitor)
    }
}

struct StrokeVisitor;

impl de::Visitor for StrokeVisitor {
    type Value = Stroke;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a string representing a single steno stroke")
    }

    fn visit_str<E>(self, s: &str) -> result::Result<Stroke, E>
        where E: de::Error
    {
        match Stroke::parse_stroke(s) {
            Ok(str) => Ok(str),
            Err(_) => Err(de::Error::invalid_value(de::Unexpected::Str(s), &self)),
        }
    }
}

static FULL_STENO: &'static str = "STKPWHRAO*EUFRPBLGTSDZ";
static NUM_STENO: &'static str = "12K3W4R50*EU6R7B8G9SDZ";
// static LEFT: u32 = 0x7f;
static MID: u32 = 0xf80;
static RIGHT: u32 = 0x3ff000;
pub static NUM: u32 = 0x400000;
static NUMS: u32 = 0x551ab;
static FSTROKE: u32 = 0x1000;
static STAR_STROKE: u32 = 0x200;

#[cfg(test)]
mod test {
    use super::*;

    /// Test the full steno conversion.  Its fairly slow, but exhaustive on the round trip.
    #[test]
    #[ignore]
    fn full_steno() {
        for i in 1 .. NUM {
            let a = Stroke(i);
            let text_a = format!("{}", a);
            // println!("Parsing: text_a: {:?}", text_a);
            let b = Stroke::parse_stroke(&text_a).unwrap();
            assert_eq!(a, b);
        }
    }
}
