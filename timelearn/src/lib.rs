//! A model for spaced repetition system learning.
//!
//! This crate maintains a database of problems, and some notions of intervals and a bit of history
//! in order to allow them to be used for spaced repetition learning.
//!
//! The key type is the `Store` which is associated with a single database file.
//!
//! A given `Problem` is simply two text strings, a question and an answer.  It is up to the user
//! of this crate to determine what these mean.  It could be as simple as text to display to the
//! user, or some encoded data to determine the correct answer.
//!
//! The client of this library should be able to ask questions, determine if the answer is correct,
//! and return a 1-4 rating of how well the user answered the question.  In some cases, it may only
//! make sense to return either a 1 for an incorrect answer, or a 4 for a correct answer.

#![deny(missing_docs)]

extern crate rand;
extern crate rusqlite;

use rand::{Rng, thread_rng};
use rusqlite::{Connection, Transaction};
use std::error;
use std::io::Write;
use std::path::Path;
use std::result;
use std::time::{SystemTime, UNIX_EPOCH};

/// A wrapper around the result type for all results returned.  Currently, the errors are just
/// boxed, and this should be improved.
pub type Result<T> = result::Result<T, Box<error::Error + Send + Sync>>;

/// A Store holds problems in a database (and holds the handle to the database).
pub struct Store {
    /// The connection to the database.
    conn: Connection,

    /// The kind, which indicates how the problems are interpreted.
    kind: String,
}

impl Store {
    /// Create a new store at the given path.  Will return an error if the database has already
    /// been created.  The `kind` is a string that can be used later to determine what kind of user
    /// interaction to use (and define the interpretation of the problems).
    pub fn create<P: AsRef<Path>>(path: P, kind: &str) -> Result<Store> {
        let mut conn = Connection::open(path)?;

        {
            let tx = conn.transaction()?;
            tx.execute("CREATE TABLE probs (id INTEGER PRIMARY KEY,
                question TEXT UNIQUE,
                answer TEXT NOT NULL)",
                         &[])?;
            tx.execute("CREATE TABLE learning (probid INTEGER PRIMARY KEY REFERENCES probs (id),
                next REAL NOT NULL,
                interval REAL NOT NULL)",
                         &[])?;
            tx.execute("CREATE TABLE config (key TEXT PRIMARY KEY, value TEXT NOT NULL)", &[])?;
            tx.execute("INSERT INTO config VALUES ('kind', ?)", &[&kind])?;
            tx.execute("CREATE INDEX learning_next ON learning (next)", &[])?;
            tx.execute("CREATE TABLE schema_version (version TEXT NOT NULL)", &[])?;
            tx.execute("CREATE TABLE log (stamp REAL NOT NULL,
                score INTEGER NOT NULL,
                probid INTEGER REFERENCES probs (id) NOT NULL)", &[])?;
            tx.execute("INSERT INTO schema_version VALUES (?)", &[&"20170709A"])?;
            tx.commit()?;
        }

        conn.execute("PRAGMA foreign_keys = ON", &[])?;

        Ok(Store {
            conn: conn,
            kind: kind.to_owned(),
        })
    }

    /// Open an existing (and ideally already populated) `Store`.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Store> {
        let conn = Connection::open(path)?;
        {
            let mut stmt = conn.prepare(" SELECT version FROM schema_version")?;
            let mut rows = stmt.query_map(&[], |row| {
                    let vers: String = row.get(0);
                    vers
                })?;
            match rows.next() {
                Some(text) => {
                    let text = text?;
                    if text != "20170709A" {
                        panic!("schema version mismatch {}", text);
                    }
                }
                None => panic!("No schema present"),
            }
            match rows.next() {
                Some(_) => panic!("Multiple rows in schema_version"),
                None => (),
            }
        }
        let kind = {
            let mut stmt = conn.prepare("SELECT value FROM config WHERE key = 'kind'")?;
            let mut rows = stmt.query_map(&[], |row| {
                let value: String = row.get(0);
                value
            })?;
            match rows.next() {
                Some(text) => text?,
                None => panic!("No kind present"),
            }
        };
        Ok(Store {
            conn: conn,
            kind: kind,
        })
    }

    /// Retrieve the kind of this Store.  This is the string given when the store was created,
    /// indicates how the problems should be interpreted.
    pub fn get_kind(&self) -> &str {
        &self.kind
    }

    /// Return a populator that can be used to more rapidly populate the data.  The population will
    /// be done within the context of a single sqlite3 database transaction.
    pub fn populate(&mut self) -> Result<Populator> {
        let tx = self.conn.transaction()?;
        Ok(Populator { tx: tx })
    }

    /// Query for the next problem that has expired.  If Some, then this is the next problem that
    /// should be asked.
    pub fn get_next(&mut self) -> Result<Option<Problem>> {
        let mut stmt = self.conn
            .prepare("
            SELECT id, question, answer, next, interval
            FROM probs JOIN learning
            WHERE probs.id = learning.probid
                AND next <= ?
            ORDER BY next
            LIMIT 1")?;
        let mut rows = stmt.query_map(&[&now()], |row| {
                Problem {
                    id: row.get(0),
                    question: row.get(1),
                    answer: row.get(2),
                    next: row.get(3),
                    interval: row.get(4),
                }
            })?;
        match rows.next() {
            Some(row) => Ok(Some(row?)),
            None => Ok(None),
        }
    }

    /// Get a problem that hasn't started being learned.  The interval and "next" will be set
    /// appropriately for a new word.
    pub fn get_new(&mut self) -> Result<Option<Problem>> {
        let mut stmt = self.conn
            .prepare("
            SELECT id, question, answer
            FROM probs
            WHERE ID NOT IN (SELECT probid FROM learning)
            ORDER BY id
            LIMIT 1")?;
        let mut rows = stmt.query_map(&[], |row| {
                Problem {
                    id: row.get(0),
                    question: row.get(1),
                    answer: row.get(2),
                    next: now(),
                    interval: 5.0,
                }
            })?;
        match rows.next() {
            Some(row) => Ok(Some(row?)),
            None => Ok(None),
        }
    }

    /// Update a word, based on a learning factor.  The scale is 1..4, with 1 being totally
    /// incorrect, and 4 being totally correct.
    pub fn update(&mut self, prob: Problem, factor: u8) -> Result<()> {
        let orig_factor = factor;
        let factor = match factor {
            1 => 0.25,
            2 => 0.9,
            3 => 1.2,
            4 => 2.2,
            _ => panic!("Invalid factor: {}", factor),
        };

        let mut prob = prob;

        let mut rng = thread_rng();
        let interval = prob.interval;
        // Compute the interval, capping with a minimum of 5 seconds.
        prob.interval = (interval * factor * rng.gen_range(0.75, 1.25)).max(5.0);
        prob.next = now() + prob.interval;

        let tx = self.conn.transaction()?;
        tx.execute("INSERT OR REPLACE INTO learning VALUES (?, ?, ?)",
                     &[&prob.id, &prob.next, &prob.interval])?;
        tx.execute("INSERT INTO log VALUES (?, ?, ?)",
                   &[&now(), &prob.id, &orig_factor])?;
        tx.commit()?;

        Ok(())
    }

    /// Retrieve statistics about the words available.
    pub fn get_counts(&self) -> Result<Counts> {
        let unlearned: i64 = self.conn
            .query_row("
                SELECT COUNT(*)
                FROM probs
                WHERE id NOT IN (SELECT probid FROM learning)",
                       &[],
                       |row| row.get(0))?;

        let cur = now();

        let active: i64 = self.conn
            .query_row("
                SELECT COUNT(*)
                FROM probs JOIN learning
                WHERE probs.id = learning.probid
                    AND next <= ?",
                       &[&cur],
                       |row| row.get(0))?;

        let later: i64 = self.conn
            .query_row("
                SELECT COUNT(*)
                FROM probs JOIN learning
                WHERE probs.id = learning.probid
                    and next > ?",
                       &[&cur],
                       |row| row.get(0))?;

        let mut interval = 1.0;
        let mut prior = 0.0;
        let buckets: Vec<_> = COUNT_BUCKETS
            .iter()
            .map(|buk| {
                interval *= buk.limit;
                let count: i64 = self.conn
                    .query_row("
                    SELECT COUNT(*)
                    FROM probs JOIN learning
                    WHERE probs.id = learning.probid
                        AND interval <= ? AND interval > ?",
                               &[&interval, &prior],
                               |row| row.get(0))
                    .unwrap();
                prior = interval;
                Bucket {
                    name: buk.name,
                    count: count as usize,
                }
            })
            .collect();

        Ok(Counts {
               active: active as usize,
               later: later as usize,
               unlearned: unlearned as usize,
               buckets: buckets,
           })
    }
}

/// Statistics about the current state of the problems.
pub struct Counts {
    /// The number of "Active" problems.  Something is considered active if it is due for learning.
    pub active: usize,
    /// The number of problems that are being learned, but aren't ready to be asked again.
    pub later: usize,
    /// The number of problems the user has never been shown.
    pub unlearned: usize,

    /// Counts of all of the problems, grouped into histogram buckets based on the learning
    /// interval of the problem.  The bucket names are a short description of the interval.
    pub buckets: Vec<Bucket>,
}

/// A single histogram bucket describing a number of problems of a given category.
pub struct Bucket {
    /// A short description of this bucket.
    pub name: &'static str,
    /// The number of problems in this bucket.
    pub count: usize,
}

struct BucketBin {
    name: &'static str,
    limit: f64,
}

static COUNT_BUCKETS: &'static [BucketBin] = &[BucketBin {
     name: "sec",
     limit: 60.0,
 },
 BucketBin {
     name: "min",
     limit: 60.0,
 },
 BucketBin {
     name: "hr",
     limit: 24.0,
 },
 BucketBin {
     name: "day",
     limit: 30.0,
 },
 BucketBin {
     name: "mon",
     limit: 1.0e30,
 }];

/// A single problem retrieved.
pub struct Problem {
    id: i64,
    /// The text of the question.  Can be an arbitrary string, encoded in a way that is meaningful
    /// to the client of this crate.
    pub question: String,
    /// Likewise, the correct answer to the question.
    pub answer: String,
    next: f64, // TODO: Make these private, and provide a query.
    interval: f64,
}

impl Problem {
    /// Create a problem.  This is generally used for testing, as it creates a problem with
    /// defaults for next, and interval.
    pub fn new(question: &str, answer: &str) -> Problem {
        Problem {
            id: -1,
            question: question.to_owned(),
            answer: answer.to_owned(),
            next: now() + 5.0,
            interval: 5.0,
        }
    }

    /// Get the Posix timestamp for when we will next ask this Problem.
    pub fn get_next(&self) -> f64 {
        self.next
    }

    /// Get the time interval, in seconds, between asks.  This will be adjusted by `update` each
    /// time a question is answered.
    pub fn get_interval(&self) -> f64 {
        self.interval
    }
}

/// The status of learning.  TODO: This doesn't belong in this crate.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Status {
    /// Continue the learning process, with the given factor.
    Continue(u8),
    /// The user has requested that we stop.
    Stopped,
}

/// A User is something that can be asked to solve a single problem.  It implements `Write` which
/// is used to prompt and present information.  The method `single` is used to ask a single
/// question, and get status back from it.  TODO: This doesn't belong in this crate.
pub trait User: Write {
    /// Ask the user to answer a single problem.  Should return a status to indicate how well the
    /// user did.
    fn single(&mut self, word: &Problem) -> Result<Status>;
}

/// A helper to populate a `Store` with `Problem`s.
pub struct Populator<'a> {
    tx: Transaction<'a>,
}

impl<'a> Populator<'a> {
    /// Add a single unlearned problem to the store.
    pub fn add_problem(&mut self, question: &str, answer: &str) -> Result<()> {
        self.tx
            .execute("INSERT INTO probs (question, answer) VALUES (?, ?)",
                     &[&question, &answer])?;
        Ok(())
    }

    /// Add a problem that is in the process of being learned.  The 'next' value is the unix time
    /// that the question should be asked again, and 'interval' is the current interval.
    ///
    /// This is used when importing problems from another system, where some of the problems may
    /// already be learned.
    pub fn add_learning_problem(&mut self,
                                question: &str,
                                answer: &str,
                                next: f64,
                                interval: f64)
                                -> Result<()> {
        self.tx
            .execute("INSERT INTO probs (question, answer) VALUES (?, ?)",
                     &[&question, &answer])?;
        self.tx
            .execute("INSERT INTO learning VALUES (?, ?, ?)",
                     &[&self.tx.last_insert_rowid(), &next, &interval])?;
        Ok(())
    }

    /// Consume the `Populator` and commit.  If the Populator is dropped without calling `commit`,
    /// any changes made by it will be rolled back.
    pub fn commit(self) -> Result<()> {
        self.tx.commit()?;
        Ok(())
    }
}

/// Get the current time in the Posix timestamp format.  This is the same time value used by the
/// 'next' field of the Problems, and can be used, for example, during population to set already
/// partially-learned problems.
pub fn now() -> f64 {
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let sec = stamp.as_secs();
    let nsec = stamp.subsec_nanos();

    sec as f64 + (nsec as f64 / 1.0e9)
}
