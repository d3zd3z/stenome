// The dictionary used for the lessons.

use serde_json;
use std::collections::BTreeMap;
use std::fs::{File, rename};
use std::path::Path;
use stroke::{self, Stroke};
use ::Result;

/// The set of words we are working on.  This represents the current state.
#[derive(Debug, Deserialize, Serialize)]
pub struct Words {
    /// The unlearned words come directly from the json problem sets.
    pub unlearned: Vec<Lesson>,
}

impl Words {
    /// Construct a new word set from internal data.
    pub fn new() -> Words {
        Words {
            unlearned: get_lessons(),
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
}

type Dict = BTreeMap<Vec<Stroke>, String>;

fn get_dict() -> Dict {
    let json = include_str!("dict-canonical.json");
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
    let json = include_str!("lessons.json");
    let mut dict: Vec<(Vec<Stroke>, String)> = get_dict().into_iter().collect();
    let infos: Vec<LessonInfo> = serde_json::from_str(json).unwrap();

    let mut result = vec![];
    // Work through the lessons in order, assigning the problems to the first lesson that makes
    // sense.
    for info in infos {
        let (with, without) = dict.into_iter().partition(|item| {
            let value = item.0[0].0;
            (value & !info.include.0 == 0) && (value & info.require.0 != 0)
        });
        result.push(Lesson {
            info: info,
            words: with,
        });
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
