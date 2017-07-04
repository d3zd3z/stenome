//! A Simple User interaction
//!
//! Implements a simple type of Q/A.  The question and answer are both text.  It presents the
//! question, waits for the user to press a key, shows the answer, and asks the user how well they
//! did.  Useful when the "answer" is a performance type of action, such as saying something out
//! loud, or playing something on a musical instrument.

use Result;
use Status;
use User;
use timelearn::Problem;

use std::io::{self, stdin, Stdin, stdout, Stdout, Write};
use termion::event::Key;
use termion::input::{Keys, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Simple {
    keys: Keys<Stdin>,
    stdout: RawTerminal<Stdout>,
}

impl Simple {
    pub fn new() -> Result<Simple> {
        Ok(Simple {
            keys: stdin().keys(),
            stdout: stdout().into_raw_mode()?,
        })
    }
}

impl User for Simple {
    fn single(&mut self, word: &Problem) -> Result<Status> {
        write!(self, "Q: {}: ", word.question)?;
        self.flush()?;

        // If the answer is the string "play", don't wait for space and an answer.
        if word.answer != "play" {
            loop {
                let c = match self.keys.next() {
                    None => panic!("End of input"),
                    Some(c) => c,
                };
                match c? {
                    Key::Esc => return Ok(Status::Stopped),
                    Key::Char(' ') => break,
                    _ => {}
                }
            }
            write!(self, "\r\n\nA: {} (1, bad, 4 - good): ", word.answer)?;
        } else {
            write!(self, "\r\n\n    (1 - bad, 4 - good): ")?;
        }
        self.flush()?;

        // Wait for the 1-4 answer or escape.
        loop {
            let c = match self.keys.next() {
                None => panic!("End of input"),
                Some(c) => c,
            };
            match c? {
                Key::Esc => return Ok(Status::Stopped),
                Key::Char(x) if x >= '1' && x <= '4' => {
                    let ch = (x as u8) - ('1' as u8) + 1;
                    write!(self, "\r\n(Learned: {})\r\n", ch)?;
                    write!(self, "\r\n\n")?;
                    return Ok(Status::Continue(ch));
                }
                _ => {}
            }
        }
    }
}

impl Write for Simple {
    fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // TODO: We could put the '\n' here after we see a return.
        self.stdout.write(buf)
    }
}
