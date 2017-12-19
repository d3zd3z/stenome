package timelearn

import "database/sql"

// A Populator wraps a database transaction to be able to populate a
// database with problems to learn.  The database should not otherwise
// be used while this is opened.
type Populator struct {
	tx *sql.Tx
}

func (t *T) Begin() (Populator, error) {
	tx, err := t.conn.Begin()
	if err != nil {
		return Populator{}, err
	}

	return Populator{tx: tx}, err
}

// Add adds a single unlearned problem to the given store.
func (p Populator) Add(question, answer string) error {
	_, err := p.tx.Exec(`INSERT INTO probs (question, answer) VALUES (?, ?)`,
		question, answer)
	return err
}

// Commit finishes adding problems to the store, by committing the
// database transaction.
func (p Populator) Commit() error {
	return p.tx.Commit()
}
