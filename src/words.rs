// The dictionary used for the lessons.

use serde_json;
use std::collections::BTreeMap;
use stroke;
use Stroke;

/// The set of words we are working on.  This represents the current state.
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

#[derive(Debug, Deserialize)]
pub struct LessonInfo {
    pub title: String,
    pub include: Stroke,
    pub require: Stroke,
    pub tags: String,
}

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
    result.push(Lesson {
        info: LessonInfo {
            title: "Uncategorized".to_string(),
            include: Stroke(stroke::NUM - 1),
            require: Stroke(0),
            tags: "Rest".to_string(),
        },
        words: dict,
    });

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn load_words() {
        let words = Words::new();
        println!("{} lessons", words.unlearned.len());
        for les in &words.unlearned {
            println!("  lesson {} words: {}", les.words.len(), les.info.title);
        }
    }
}
