// A model for spaced repetition system learning.
//
// This package maintains a database of problems, and some notions of
// intervals and a bit of history in order to allow them to be used
// for spaced repetition learning.
//
// The key type is the `T` which is associated with a single
// database file.
//
// A given `Problem` is simply two text strings, a question and an
// answer.  It is up to the user of this package to determine what
// these mean.  It could be as simple as text to display to the user,
// or some encoded data to determine the correct answer.
//
// The client of this package should be able to ask questions,
// determine if the answer is correct, and return a 1-4 rating of how
// well the user answered the question.  In some cases, it may only
// make sense to return either a 1 for an incorrect answer, or a 4 for
// a correct answer.
package timelearn // import "davidb.org/x/stenome/timelearn"

import (
	"database/sql"
	"fmt"
	"log"
	"time"

	_ "github.com/mattn/go-sqlite3"
)

type T struct {
	conn *sql.DB // The connection to the database
	kind string  // The kind, indicates how problems are interpreted.

	// Return the current time.  For testing, can be set to
	// something predictible.
	now func() time.Time
}

var (
	schema = []string{
		`CREATE TABLE probs (id INTEGER PRIMARY KEY,
		question TEXT UNIQUE,
		answer TEXT NOT NULL)`,
		`CREATE TABLE learning (probid INTEGER PRIMARY KEY REFERENCES probs (id),
		next REAL NOT NULL,
		interval REAL NOT NULL)`,
		`CREATE TABLE config (key TEXT PRIMARY KEY, value TEXT NOT NULL)`,
		`CREATE INDEX learning_next ON learning (next)`,
		`CREATE TABLE schema_version (version TEXT NOT NULL)`,
		`CREATE TABLE log (stamp REAL NOT NULL,
		score INTEGER NOT NULL,
		probid INTEGER REFERENCES probs (id) NOT NULL)`,
	}
	schema_version = "20170709A"
)

// Create creates a new store at the given path.  Will return an error
// if the database has already been created.  The `kind` is a string
// that can be used later to determine what kind of user interaction
// to use (and that defines the interpretation of the problems).
func Create(path, kind string) (*T, error) {
	db, err := sql.Open("sqlite3", path)
	if err != nil {
		return nil, err
	}

	for _, line := range schema {
		_, err = db.Exec(line)
		if err != nil {
			log.Printf("SQL error in %q", line)
			db.Close()
			return nil, err
		}
	}

	// Set the kind.
	_, err = db.Exec(`INSERT INTO config VALUES ('kind', ?)`, kind)
	if err != nil {
		db.Close()
		return nil, err
	}

	// And lastly the schema version.
	_, err = db.Exec(`INSERT INTO schema_version VALUES (?)`, schema_version)
	if err != nil {
		db.Close()
		return nil, err
	}

	return &T{
		conn: db,
		kind: kind,
		now:  time.Now,
	}, nil
}

// Open an existing database.
func Open(path string) (*T, error) {
	conn, err := sql.Open("sqlite3", path)
	if err != nil {
		return nil, err
	}

	// Read the schema version, handling it properly.
	var version string
	err = conn.QueryRow("SELECT version FROM schema_version").Scan(&version)
	if err != nil {
		conn.Close()
		return nil, err
	}
	if version != schema_version {
		conn.Close()
		return nil, fmt.Errorf("Schema version mismatch, database is %q, expecting %q",
			version, schema_version)
	}

	// Read the kind from the database.
	var kind string
	err = conn.QueryRow("SELECT value FROM config WHERE key = 'kind'").Scan(&kind)
	if err != nil {
		conn.Close()
		return nil, err
	}

	return &T{
		conn: conn,
		kind: kind,
		now:  time.Now,
	}, nil
}

func (t *T) Close() error {
	return t.conn.Close()
}

func (t *T) Kind() string {
	return t.kind
}

// For debug and testing, override the query for the current time with
// something that is configurable.
func (t *T) TestSetNowFunc(now func() time.Time) {
	t.now = now
}
