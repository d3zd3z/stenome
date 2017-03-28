// Learning.

use term::Term;
use words::{LearnWord, Words};

use std::io::Write;

pub struct Learn {
    words: Words,
    term: Term,
}

impl Learn {
    pub fn new(words: Words, term: Term) -> Learn {
        Learn {
            words: words,
            term: term,
        }
    }

    // Consume the learner, returning the wordset contained in it.
    pub fn into_words(self) -> Words {
        self.words
    }

    pub fn run(&mut self) {
        loop {
            // TODO: Check for things that have expired we need to learn.

            let mut word = match self.words.get_unlearned() {
                None => {
                    println!("No more words to learn\r");
                    return;
                }
                Some(word) => word,
            };

            if self.single(&mut word) == Status::Stopped {
                break;
            }
        }
    }

    // Learn a single word, updating its timing information based on how well it was learned.
    fn single(&mut self, word: &mut LearnWord) -> Status {
        // Loop until we get it right.
        if word.strokes.len() != 1 {
            panic!("TODO");
        }
        loop {
            write!(self.term, "{:20}: ", word.english).unwrap();
            self.term.flush().unwrap();
            let stroke = match self.term.read_stroke().unwrap() {
                None => return Status::Stopped,
                Some(st) => st,
            };
            if stroke == word.strokes[0] {
                writeln!(self.term, "✓ {}\r", stroke).unwrap();
                self.term.flush().unwrap();
                break;
            } else {
                // TODO: Should wait for a correction.
                writeln!(self.term, "✗ {} ({})\r", stroke, word.strokes[0]).unwrap();
                self.term.flush().unwrap();
            }
        }
        Status::Continue
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Status {
    Continue,
    Stopped,
}
