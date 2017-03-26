// The dictionary used for the lessons.

use serde_json;
use std::collections::BTreeMap;
use Stroke;

type Dict = BTreeMap<Vec<Stroke>, String>;

pub fn get_dict() -> Dict {
    let json = include_str!("dict-canonical.json");
    let main: BTreeMap<String, String> = serde_json::from_str(json).unwrap();
    let mut result = BTreeMap::new();
    for (k, v) in main {
        result.insert(Stroke::parse_strokes(&k).unwrap(), v);
    }
    result
}

#[derive(Debug, Deserialize)]
pub struct Lesson {
    title: String,
    include: Stroke,
    require: Stroke,
    tags: String,
}

pub fn get_lessons() -> Vec<Lesson> {
    let json = include_str!("lessons.json");
    serde_json::from_str(json).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn load_dict() {
        let dict = get_dict();
        println!("{} nodes to learn", dict.len());
    }

    #[test]
    fn load_lessons() {
        let lessons = get_lessons();
        println!("{} lessons to cover", lessons.len());
    }
}
