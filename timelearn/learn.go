package timelearn

import (
	"context"
	"math"
	"math/rand"
	"time"
)

// A problem is a single problem retrieved.
type Problem struct {
	id       int64
	Question string        // The question to be asked.
	Answer   string        // The correct answer.
	Next     time.Time     // When to next ask this question.
	Interval time.Duration // Current interval to next question
}

// GetNexts queries for `count` upcoming problems that are ready to be
// asked.  Will return an array of problems, with element 0 being the
// next problem that should be asked.
func (t *T) GetNexts(count int) ([]*Problem, error) {
	rows, err := t.conn.QueryContext(context.Background(), `
		SELECT id, question, answer, next, interval
		FROM probs JOIN learning
		WHERE probs.id = learning.probid
			AND next <= ?
		ORDER BY next
		LIMIT ?`, timeToDb(t.now()), count)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var result []*Problem

	for rows.Next() {
		var next float64
		var interval float64
		var p Problem
		err = rows.Scan(&p.id, &p.Question, &p.Answer, &next, &interval)
		if err != nil {
			return nil, err
		}

		p.Next = dbToTime(next)
		p.Interval = dbToDur(interval)

		result = append(result, &p)
	}

	// If we got no rows back, fetch an unlearned problem.  It
	// doesn't make any sense to return more than one, because
	// they will usually be incorrect (time will pass causing
	// other problems to become ready).
	if len(result) == 0 {
		prob, err := t.GetNew()
		if err != nil {
			return nil, err
		}

		if prob != nil {
			result = append(result, prob)
		}
	}

	return result, nil
}

// Get a problem that hasn't started being learned.  The interval and
// next will be set appropriately for a new problem.
// TODO: Set the interval based on a configurable value, as the
// default depends on the problem types.
func (t *T) GetNew() (*Problem, error) {
	var p Problem
	err := t.conn.QueryRow(`
		SELECT id, question, answer
		FROM probs
		WHERE ID NOT IN (SELECT probid FROM learning)
		ORDER BY id
		LIMIT 1`).Scan(&p.id, &p.Question, &p.Answer)
	if err != nil {
		return nil, err
	}

	p.Next = t.now()
	p.Interval = 5 * time.Second

	return &p, nil
}

// Update updates a problem, based on a learning factor.  The scale is
// 1-4, with 1 being totally incorrect, and 4 being totally correct.
func (t *T) Update(prob *Problem, factor int) error {
	var adj float64
	switch factor {
	case 1:
		adj = 0.25
	case 2:
		adj = 0.9
	case 3:
		adj = 1.2
	case 4:
		adj = 2.2
	default:
		panic("Invalid factor, should be 1-4")
	}

	// TODO: Cap this based on the minimum interval, not this
	// arbitrary 5 second value.
	tweak := rand.Float64()*(1.25-0.75) + 0.75
	new_interval := time.Duration(float64(prob.Interval) * adj * tweak)
	if new_interval < 5*time.Second {
		new_interval = 5 * time.Second
	}

	prob.Interval = new_interval
	prob.Next = t.now().Add(prob.Interval)

	tx, err := t.conn.Begin()
	if err != nil {
		return err
	}

	_, err = tx.Exec(`INSERT OR REPLACE INTO learning VALUES (?, ?, ?)`,
		prob.id, timeToDb(prob.Next), durToDb(prob.Interval))
	if err != nil {
		return err
	}

	_, err = tx.Exec(`INSERT INTO log VALUES (?, ?, ?)`,
		timeToDb(t.now()), prob.id, factor)
	if err != nil {
		return err
	}
	err = tx.Commit()
	if err != nil {
		return err
	}

	return nil
}

// Convert time to a float64 so that it can be round-tripped to the
// database.
func timeToDb(t time.Time) float64 {
	return float64(t.Unix()) + float64(t.UnixNano())/1.0e9
}

// Convert time from float64 for round-tripping
func dbToTime(t float64) time.Time {
	if t <= 0.0 {
		panic("Don't expect negative time")
	}

	sec := math.Trunc(t)
	nsec := (t - sec) * 1.0e9

	return time.Unix(int64(sec), int64(nsec))
}

func durToDb(d time.Duration) float64 {
	return float64(d) / 1.0e9
}

func dbToDur(d float64) time.Duration {
	return time.Duration(d * 1.0e9)
}
