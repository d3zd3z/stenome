package learn

import (
	"davidb.org/x/stenome/timelearn"
)

// The UI is responsible for asking the user a question, and
// determining if they got it right.  The prob argument will always
// point to the problem to learn.  'next' may point to an upcoming
// problem.
type UI interface {
	Single(prob, next *timelearn.Problem) (int, error)
	Close() error
}

// Mapping holding all of the UIs that can be learned.  The mapping
// holds a generator to generate the UI.
var allUI = map[string]func() (UI, error){
	"simple": newSimpleUI,
}

// Register adds a new user interface.  Any database with the given
// kind will use this function to make a new UI for it.
func Register(kind string, gen func() (UI, error)) {
	allUI[kind] = gen
}
