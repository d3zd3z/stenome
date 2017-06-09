/// Import the legacy json data into the new timelearn database.

use Result;
use serde_json;
use std::fs::File;
use std::path::Path;
use stroke::Stroke;
use timelearn::Store;

/// This structure should match the json used before.
#[derive(Debug, Deserialize)]
pub struct Words {
    pub unlearned: Vec<Lesson>,
    pub learning: Vec<LearnWord>,
}

#[derive(Debug, Deserialize)]
pub struct Lesson {
    pub info: LessonInfo,
    pub words: Vec<(Vec<Stroke>, String)>,
}

#[derive(Debug, Deserialize)]
pub struct LessonInfo {
    pub title: String,
    pub include: Stroke,
    pub require: Stroke,
    pub tags: String,
}

#[derive(Debug, Deserialize)]
pub struct LearnWord {
    pub strokes: Vec<Stroke>,
    pub english: String,
    pub next: f64,
    pub interval: f64,
}

impl Words {
    /// Load a wordset from the given file.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Words> {
        let fd = File::open(path)?;
        let result = serde_json::from_reader(&fd)?;
        Ok(result)
    }

    /// Create a database, and add all of these words into it.
    pub fn create_db<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let mut st = Store::create(path)?;
        let mut pop = st.populate()?;

        // Start with the words being learned. The order will be based on the current intervals, so
        // not exactly matching the initial order, but this should at least get these before the
        // unlearned words.
        for w in &self.learning {
            let strokes = Stroke::slashed_str(&w.strokes);
            pop.add_learning_problem(&w.english, &strokes, w.next, w.interval)?;
        }

        // Then add all of the unlearned words.
        for les in &self.unlearned {
            for w in &les.words {
                let strokes = Stroke::slashed_str(&w.0);
                pop.add_problem(&w.1, &strokes)?;
            }
        }

        pop.commit()?;

        Ok(())
    }
}
