// A terminal.

use Result;
use stroke::Stroke;

use std::io::{self, Stdin, stdin, Stdout, stdout, Write};
use termion::event::Key;
use termion::input::{Keys, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Term {
    keys: Keys<Stdin>,
    stdout: RawTerminal<Stdout>,
}

impl Term {
    pub fn new() -> Result<Term> {
        Ok(Term {
            keys: stdin().keys(),
            stdout: stdout().into_raw_mode()?,
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
                            Ok(st) => return Ok(Some(st)),
                            Err(e) => {
                                writeln!(self, "Invalid stroke received: {:?}\r", e)?;
                                self.flush()?;
                            }
                        }
                        chars.clear();
                    }
                }
                Key::Char(ch) => chars.push(ch),
                other => {
                    writeln!(self, "Unknown: {:?}\r", other)?;
                    self.flush()?;
                }
            }
        }
    }
}

impl Write for Term {
    fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // TODO: We could put the '\n' here after we see a return.
        self.stdout.write(buf)
    }
}
