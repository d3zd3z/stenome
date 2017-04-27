// The dictionary used for the lessons.

use rand::{Rng, thread_rng};
use serde_json;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fs::{File, rename};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use stroke::{self, Stroke};
use ::Result;

/// The set of words we are working on.  This represents the current state.
#[derive(Debug, Deserialize, Serialize)]
pub struct Words {
    /// The unlearned words come directly from the json problem sets.
    pub unlearned: Vec<Lesson>,

    pub learning: Vec<LearnWord>,
}

impl Words {
    /// Construct a new word set from internal data.
    pub fn new() -> Words {
        Words {
            unlearned: get_lessons(),
            learning: vec![],
        }
    }

    /// Load a wordset from the given file.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Words> {
        let fd = File::open(path.as_ref())?;
        let result = serde_json::from_reader(&fd)?;
        Ok(result)
    }

    /// Save the wordset to a given file.
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();
        let tmp = path.with_extension("tmp");
        {
            let mut fd = File::create(&tmp)?;
            // serde_json::to_writer_pretty(&mut fd, self)?;
            serde_json::to_writer(&mut fd, self)?;
            // serde_yaml::to_writer(&mut fd, self)?;
            // rmp_serde::encode::write(&mut fd, self)?;
        }
        rename(tmp, path)?;
        Ok(())
    }

    /// Extract a new word to learn.  Returns None if there are no more words.
    pub fn get_unlearned(&mut self) -> Option<LearnWord> {
        let mut lesson = match self.unlearned.pop() {
            None => return None,
            Some (l) => l,
        };

        let (strokes, english) = lesson.words.pop().expect("Words in list");

        let result = LearnWord {
            strokes: strokes,
            english: english,
            next: now() + 5.0,
            interval: 5.0,
        };

        // If there are more words in this lesson, push the lesson back.
        if !lesson.words.is_empty() {
            self.unlearned.push(lesson);
        }

        Some(result)
    }

    /// Extract a word to be learned, if were ready for it.
    pub fn get_learned(&mut self) -> Option<LearnWord> {
        match self.learning.pop() {
            None => None,
            Some(w) => {
                if now() >= w.next {
                    Some(w)
                } else {
                    self.learning.push(w);
                    None
                }
            }
        }
    }

    /// Push a word back into the unlearned word list.
    pub fn push_word(&mut self, word: LearnWord) {
        self.learning.push(word);
        self.learning.sort_by(word_cmp);
    }

    /// Return a count of the number of words that are pending.
    pub fn get_counts(&self) -> Counts {
        let n = now();

        // Divide all of the upcoming entries into buckets.
        let mut counts = vec![0; COUNT_BUCKETS.len()];

        for word in &self.learning {
            let mut v = word.interval;
            for (num, b) in COUNT_BUCKETS.iter().enumerate() {
                if v < b.limit {
                    counts[num] += 1;
                    break;
                }
                v /= b.limit;
            }
        }

        let buckets = COUNT_BUCKETS.iter().zip(counts).map(|(aa, bb)| Bucket {
            name: aa.name,
            count: bb,
        }).collect();

        Counts {
            active: self.learning.iter().filter(|x| x.next < n).count(),
            later: self.learning.iter().filter(|x| x.next >= n).count(),
            unlearned: self.unlearned.iter().map(|x| x.words.len()).sum(),

            buckets: buckets,
        }
    }
}

pub struct Counts {
    pub active: usize,
    pub later: usize,
    pub unlearned: usize,

    // Buckets for later.  The sum of all of these should match the `later+active` value above.
    pub buckets: Vec<Bucket>,
}

pub struct Bucket {
    pub name: &'static str,
    pub count: usize,
}

struct BucketBin {
    name: &'static str,
    limit: f64,
}

static COUNT_BUCKETS: &'static [BucketBin] = &[
    BucketBin{ name: "sec", limit: 60.0 },
    BucketBin{ name: "min", limit: 60.0 },
    BucketBin{ name: "hr", limit: 24.0 },
    BucketBin{ name: "day", limit: 30.0 },
    BucketBin{ name: "mon", limit: 1.0e30 },
];

#[derive(Debug, Serialize, Deserialize)]
pub struct LearnWord {
    pub strokes: Vec<Stroke>,
    pub english: String,
    pub next: f64,
    pub interval: f64,
}

impl LearnWord {
    pub fn correct(&mut self) {
        self.adjust(2.2)
    }

    pub fn incorrect(&mut self) {
        self.adjust(0.25)
    }

    fn adjust(&mut self, factor: f64) {
        let mut rng = thread_rng();
        let interval = self.interval;
        // Compute the interval, capping with a minimum of 5 seconds.
        self.interval = (interval * factor * rng.gen_range(0.75, 1.25)).max(5.0);
        self.next = now() + self.interval;
    }
}

// Comparison is reversed to make it easy to have the newest at the end.
fn word_cmp(a: &LearnWord, b: &LearnWord) -> Ordering {
    b.next.partial_cmp(&a.next).unwrap()
}

// Get the current time as ticks.
pub fn now() -> f64 {
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let sec = stamp.as_secs();
    let nsec = stamp.subsec_nanos();

    sec as f64 + (nsec as f64 / 1.0e9)
}

type Dict = BTreeMap<Vec<Stroke>, String>;

fn get_dict() -> Dict {
    let json = include_str!("../../etc/dict-canonical.json");
    let main: BTreeMap<String, String> = serde_json::from_str(json).unwrap();
    let mut result = BTreeMap::new();
    for (k, v) in main {
        result.insert(Stroke::parse_strokes(&k).unwrap(), v);
    }
    result
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LessonInfo {
    pub title: String,
    pub include: Stroke,
    pub require: Stroke,
    pub tags: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Lesson {
    pub info: LessonInfo,
    pub words: Vec<(Vec<Stroke>, String)>,
}

fn get_lessons() -> Vec<Lesson> {
    let json = include_str!("../../etc/lessons.json");
    let mut dict: Vec<(Vec<Stroke>, String)> = get_dict().into_iter().collect();
    let infos: Vec<LessonInfo> = serde_json::from_str(json).unwrap();

    let mut result = vec![];
    // Work through the lessons in order, assigning the problems to the first lesson that makes
    // sense.
    for info in infos {
        let (mut with, without): (Vec<_>, Vec<_>) = dict.into_iter().partition(|item| {
            let value = item.0[0].0;
            (value & !info.include.0 == 0) && (value & info.require.0 != 0)
        });
        with.reverse();
        if !with.is_empty() {
            result.push(Lesson {
                info: info,
                words: with,
            });
        }
        dict = without;
    }

    // Put the remaining words in a separate lesson.  The website will never ask for these, but we
    // should still learn them.
    // The 'require' can't ever be correct here, but we'll include something so that the saved data
    // will parse.
    result.push(Lesson {
        info: LessonInfo {
            title: "Uncategorized".to_string(),
            include: Stroke(stroke::NUM - 1),
            require: Stroke(1),
            tags: "Rest".to_string(),
        },
        words: dict,
    });

    result.reverse();

    result
}

#[cfg(test)]
mod test {
    use super::*;
    use tempdir::TempDir;

    #[test]
    fn load_words() {
        let words = Words::new();
        println!("{} lessons", words.unlearned.len());
        for les in &words.unlearned {
            println!("  lesson {} words: {}", les.words.len(), les.info.title);
        }

        let tmp = TempDir::new("words-test").expect("create temp dir");
        let json_path = tmp.path().join("sample.json");
        words.save(&json_path).unwrap();

        let w2 = Words::load(&json_path).unwrap();
        assert_eq!(words.unlearned.len(), w2.unlearned.len());
    }
}
