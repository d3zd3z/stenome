//! SQL storage of the problem set.

use Result;
use stroke::Stroke;
use words::Words;
use rusqlite::Connection;
use std::path::Path;

use super::json::now;

/// Initialize the database.
pub struct Store {
    /// The connection to the database.
    conn: Connection,
}

impl Store {
    pub fn create<P: AsRef<Path>>(path: P) -> Result<Store> {
        let mut conn = Connection::open(path)?;

        {
            let tx = conn.transaction()?;
            tx.execute("CREATE TABLE words (id INTEGER PRIMARY KEY,
                            strokes TEXT UNIQUE,
                            english TEXT NOT NULL)", &[])?;
            tx.execute("CREATE TABLE learning (wordid INTEGER NOT NULL REFERENCES words (id),
                            next REAL NOT NULL,
                            interval REAL NOT NULL)", &[])?;
            tx.execute("CREATE TABLE schema_version (version TEXT NOT NULL)", &[])?;
            tx.execute("INSERT INTO schema_version VALUES (?)", &[&"20170408B"])?;
            tx.commit()?;
        }

        Ok(Store {
            conn: conn,
        })
    }

    pub fn add_words(&mut self, words: &Words) -> Result<()> {
        let tx = self.conn.transaction()?;

        // Start with the words being learned. The order will be based on the
        // current intervals, so not exactly matching the initial order, but
        // this should at least get these before the unlearned words.
        for w in &words.learning {
            let strokes = Stroke::slashed_str(&w.strokes);
            tx.execute("INSERT INTO words (strokes, english) VALUES (?, ?)",
                &[&strokes, &w.english])?;
            tx.execute("INSERT INTO learning VALUES (?, ?, ?)",
                &[&tx.last_insert_rowid(), &w.next, &w.interval])?;
        }

        // Add all of the unlearned words.
        for les in &words.unlearned {
            for w in &les.words {
                let strokes = Stroke::slashed_str(&w.0);
                tx.execute("INSERT INTO words (strokes, english) VALUES (?, ?)",
                    &[&strokes, &w.1])?;
            }
        }

        tx.commit()?;
        Ok(())
    }

    /// Query for the next word we should ask the user about. First, look for words where 'next'
    /// has expired. After that, determine how far away the next word is. If it is reasonably near,
    /// and we don't want to be adding words, just ask it, otherwise bring in a new word.
    pub fn get_next(&mut self) -> Result<String> {
        // Retrieve the soonest "next" word.
        let mut stmt = self.conn.prepare("
                SELECT strokes, english, next, interval
                FROM words JOIN learning
                WHERE words.id = learning.wordid
                  AND next < ?
                ORDER BY next
                LIMIT 1")?;
        let row: (String, String, f64, f64) = stmt.query_row(&[&now()], |row| {
            (row.get(0), row.get(1), row.get(2), row.get(3))
        })?;
        println!("{:?}", row);
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::Words;

    #[test]
    fn make_db() {
        let mut st = Store::create("sample.db").unwrap();
        // let words = Words::new();
        let words = Words::load("learning.json").unwrap();
        st.add_words(&words);
    }
}
