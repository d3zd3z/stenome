package timelearn_test

import (
	"fmt"
	"io/ioutil"
	"os"
	"path/filepath"
	"reflect"
	"strings"
	"testing"
	"time"

	"davidb.org/x/stenome/timelearn"
)

func TestCreate(t *testing.T) {
	tdir, err := ioutil.TempDir("", "timelearn-")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tdir)

	dbName := filepath.Join(tdir, "sample.db")

	db, err := timelearn.Create(dbName, "simple")
	if err != nil {
		t.Fatal(err)
	}
	db.Close()

	db, err = timelearn.Open(dbName)
	if err != nil {
		t.Fatal(err)
	}
	defer db.Close()

	if db.Kind() != "simple" {
		t.Fatalf("Retrieved kind mismatch: got %q, want %q", db.Kind(), "simple")
	}
}

// Try creating some problems, and learning some things.
func TestLearn(t *testing.T) {
	tdir, err := ioutil.TempDir("", "timelearn-")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tdir)

	// Debug time.
	now := time.Now()

	db, err := timelearn.Create(filepath.Join(tdir, "haha.db"), "simple")
	if err != nil {
		t.Fatal(err)
	}
	defer db.Close()

	db.TestSetNowFunc(func() time.Time {
		return now
	})

	pop, err := db.Begin()
	if err != nil {
		t.Fatal(err)
	}

	for i := 1; i <= 10; i++ {
		err = pop.Add(fmt.Sprintf("Question #%d", i),
			fmt.Sprintf("Answer #%d", i))
		if err != nil {
			t.Fatal(err)
		}
	}
	if err = pop.Commit(); err != nil {
		t.Fatal(err)
	}

	prob, err := db.GetNexts(2)
	if err != nil {
		t.Fatal(err)
	}
	if len(prob) < 1 {
		t.Fatalf("GetNexts didn't return a problem")
	}

	// Act as if we learned it well.
	err = db.Update(prob[0], 4)
	if err != nil {
		t.Fatal(err)
	}

	// Advance by 20 seconds, and we should be the same problem.
	now = now.Add(20 * time.Second)

	prob2, err := db.GetNexts(2)
	if err != nil {
		t.Fatal(err)
	}
	if len(prob) < 1 {
		t.Fatalf("GetNexts didn't return a problem")
	}

	// The problems may get = " NEW" appended to them.
	if !strings.HasSuffix(prob[0].Question, " NEW") {
		prob[0].Question += " NEW"
	}

	if !strings.HasSuffix(prob2[0].Question, " NEW") {
		prob2[0].Question += " NEW"
	}

	if prob[0].Question != prob2[0].Question {
		t.Fatalf("New problem returned isn't one we expect to learn: %q, %q",
			prob[0].Question, prob2[0].Question)
	}

	counts, err := db.GetCounts()
	if err != nil {
		t.Fatal(err)
	}

	goodCounts := &timelearn.Counts{
		Active:    1,
		Later:     0,
		Unlearned: 9,
		Buckets: []timelearn.Bucket{
			timelearn.Bucket{"sec", 1},
			timelearn.Bucket{"min", 0},
			timelearn.Bucket{"hr", 0},
			timelearn.Bucket{"day", 0},
			timelearn.Bucket{"mon", 0},
		},
	}

	if !reflect.DeepEqual(counts, goodCounts) {
		t.Fatalf("Statistics mismatch %+v != %+v", counts, goodCounts)
	}
}
