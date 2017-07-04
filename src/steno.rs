//! Manage learning from a Steno device.

use Result;
use stroke::Stroke;
use Status;
use humanize_time;

use std::io::{self, Stdin, stdin, Stdout, stdout, Write};
use termion::event::Key;
use termion::input::{Keys, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::color;
use timelearn::Problem;

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

pub struct Single<'t, 'w> {
    user: &'t mut Steno,
    word: &'w Problem,
    strokes: Vec<Stroke>,
    input: Vec<Stroke>,
    errors: u32,
}

impl<'t, 'w> Single<'t, 'w> {
    pub fn new<'tt, 'ww>(user: &'tt mut Steno, word: &'ww Problem) -> Single<'tt, 'ww> {
        Single {
            user: user,
            word: word,
            strokes: Stroke::parse_strokes(&word.answer).unwrap(),
            input: vec![],
            errors: 0,
        }
    }

    fn prompt(&mut self) {
        write!(self.user,
               "\r\x1b[J{:20}: {}{}",
               self.word.question,
               if self.strokes == self.input {
                   if self.errors == 0 { '✓' } else { '✗' }
               } else {
                   ' '
               },
               slashed(&self.input, &self.strokes))
                .unwrap();
        if self.errors > 0 {
            write!(self.user, "  ({})", slashed(&self.strokes, &self.strokes)).unwrap();
        }
        self.user.flush().unwrap();
    }

    pub fn run(&mut self) -> Status {
        let result;
        loop {
            self.prompt();
            if self.strokes == self.input {
                result = Status::Continue(if self.errors > 0 { 1 } else { 4 });
                break;
            }

            let stroke = match self.user.read_stroke().unwrap() {
                None => {
                    result = Status::Stopped;
                    break;
                }
                Some(st) => st,
            };
            if stroke.is_star() {
                self.input.pop();
            } else {
                self.input.push(stroke);
                let pos = self.input.len();
                if pos > self.strokes.len() || self.input[pos - 1] != self.strokes[pos - 1] {
                    self.errors += 1;
                }
            }
        }

        match result {
            Status::Continue(_) => {
                writeln!(self.user,
                         "\r\nNew interval {}\r",
                         humanize_time(self.word.get_interval()))
                        .unwrap();
            }
            Status::Stopped => writeln!(self.user, "\r").unwrap(),
        }
        self.user.flush().unwrap();
        result
    }
}

// Generate a slash separated version of the given stroke list.
fn slashed(strokes: &[Stroke], expected: &[Stroke]) -> String {
    let mut buf = vec![];
    let mut first = true;

    for (i, st) in strokes.iter().enumerate() {
        if !first {
            buf.push(b'/');
        }
        first = false;

        let correct = expected.get(i) == Some(st);
        if !correct {
            write!(&mut buf,
                   "{}{}",
                   color::Bg(color::LightRed),
                   color::Fg(color::Black))
                    .unwrap();
        }
        write!(&mut buf, "{}", st).unwrap();
        if !correct {
            write!(&mut buf,
                   "{}{}",
                   color::Bg(color::Reset),
                   color::Fg(color::Reset))
                    .unwrap();
        }
    }
    String::from_utf8(buf).unwrap()
}
