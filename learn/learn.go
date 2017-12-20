package learn

import (
	"bytes"
	"fmt"

	"davidb.org/x/stenome/timelearn"
)

// Drill quizzes the user on the problems in the given timelearn.T
// task.  How the questions and answers are presented is determined by
// the 'kind' stored when the timelearn.T database was created.  New
// learners can be added by calling Register.
func Drill(tl *timelearn.T) error {
	kind := tl.Kind()
	uigen, ok := allUI[kind]
	if !ok {
		return fmt.Errorf("Unknown learning type: %s", kind)
	}

	ui, err := uigen()
	if err != nil {
		return err
	}

	learn := learner{
		ui: ui,
		tl: tl,
	}
	return learn.run()
}

type learner struct {
	ui UI
	tl *timelearn.T
}

// Loop on learning until we either have nothing left to learn, or the
// user tells us to stop.
func (lr *learner) run() error {
	for {
		probs, err := lr.tl.GetNexts(2)
		if err != nil {
			return err
		}

		if len(probs) == 0 {
			fmt.Printf("No more problems to learn\n")
			return nil
		}

		var next *timelearn.Problem
		if len(probs) > 1 {
			next = probs[1]
		}

		status, err := lr.single(probs[0], next)
		if err != nil {
			return err
		}

		if status == 0 {
			fmt.Printf("\nStopped\n")
			return nil
		}

		err = lr.tl.Update(probs[0], status)
		if err != nil {
			return err
		}
	}
}

func (lr *learner) single(prob, next *timelearn.Problem) (int, error) {
	counts, err := lr.tl.GetCounts()
	if err != nil {
		return 0, err
	}

	fmt.Printf("\nActive: %d, Later: %d, Unlearned: %d, Interval: %s\n",
		counts.Active, counts.Later, counts.Unlearned, Humanized(prob.Interval))

	active := 0
	learned := 0
	for _, b := range counts.Buckets {
		fmt.Printf("  %-4s: %d %s\n", b.Name, b.Count,
			stars(65, b.Count, counts.Active+counts.Later))
		// This shouldn't be done by string matching, it is
		// fragile.
		if b.Name == "day" || b.Name == "mon" {
			learned += b.Count
		} else {
			active += b.Count
		}
	}
	fmt.Printf("  active : %d\n", active)
	fmt.Printf("  learned: %d\n", learned)

	return lr.ui.Single(prob, next)
}

func stars(len, value, total int) string {
	var buf bytes.Buffer

	buf.WriteRune('|')
	thresh := float64(value) / float64(total) * float64(len)
	for i := 0; i < len; i++ {
		if float64(i) < thresh {
			buf.WriteRune('*')
		} else {
			buf.WriteRune(' ')
		}
	}
	buf.WriteRune('|')
	return buf.String()
}

type Humanized int64

var _ fmt.Formatter = Humanized(2)

func (t Humanized) Format(f fmt.State, c rune) {
	val := float64(t) / 1.0e9
	for _, unit := range allUnits {
		if val < unit.div {
			fmt.Fprintf(f, "%.1f %s", val, unit.name)
			return
		}
		val /= unit.div
	}
	panic("Time way out of bounds")
}

var allUnits = []struct {
	name string
	div  float64
}{
	{"seconds", 60.0},
	{"minutes", 60.0},
	{"hours", 24.0},
	{"days", 30.0},
	{"months", 12.0},
	{"years", 1.0e9},
}
