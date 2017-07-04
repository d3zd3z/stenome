//! Manage learning from a Steno device.

use Result;
use stroke::Stroke;

use std::io::{self, Stdin, stdin, Stdout, stdout, Write};
use termion::event::Key;
use termion::input::{Keys, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Steno {
    keys: Keys<Stdin>,
    stdout: RawTerminal<Stdout>,

    // Count of characters sent for each stroke, to match with backspaces.
    counts: Vec<usize>,
}

impl Steno {
    pub fn new() -> Result<Steno> {
        Ok(Steno {
               keys: stdin().keys(),
               stdout: stdout().into_raw_mode()?,
               counts: vec![],
           })
    }

    /// Attempt to read a stroke from the terminal, or returns None if there are no strokes
    /// available.
    pub fn read_stroke(&mut self) -> Result<Option<Stroke>> {
        let mut chars = String::new();

        loop {
            let c = match self.keys.next() {
                None => panic!("End of input"),
                Some(c) => c,
            };
            match c? {
                Key::Esc => return Ok(None),
                Key::Char(' ') => {
                    if !chars.is_empty() {
                        match Stroke::parse_stroke(&chars) {
                            Ok(st) => {
                                self.counts.push(1 + chars.len());
                                if self.counts.len() > 50 {
                                    self.counts.remove(1);
                                }
                                chars.clear();
                                return Ok(Some(st));
                            }
                            Err(e) => {
                                writeln!(self, "Invalid stroke received: {:?}\r", e)?;
                                self.flush()?;
                            }
                        }
                        chars.clear();
                    }
                }
                Key::Char(ch) => chars.push(ch),
                Key::Backspace => {
                    match self.counts.pop() {
                        None => {
                            writeln!(self, "Extraneous backspace")?;
                            self.flush()?;
                        }
                        Some(1) => {
                            // Fully backspaced, return the backspace.
                            return Ok(Some(Stroke::make_star()));
                        }
                        Some(count) => self.counts.push(count - 1),
                    }
                }
                other => {
                    writeln!(self, "Unknown: {:?}\r", other)?;
                    self.flush()?;
                }
            }
        }
    }
}

impl Write for Steno {
    fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // TODO: We could put the '\n' here after we see a return.
        self.stdout.write(buf)
    }
}
