package gen

import (
	"encoding/json"
	"fmt"

	"davidb.org/x/stenome/timelearn"
)

// Replace the scales in the given problem set with newly generated
// ones.  In order for the results to be meaningful, we rely on the
// "INTEGER PRIMARY KEY" field in SQLITE3 to always be 1 larger than
// the current largest value (starting at 1 for an empty table).
func GenScales(tl *timelearn.T) error {

	tx, err := tl.Begin()
	if err != nil {
		return err
	}

	err = tx.Wipe()
	if err != nil {
		tx.Rollback()
		return err
	}

	for _, prac := range scalePractice {
		for _, hands := range prac.hands {
			for _, base := range allKeys {
				err := GenScale(tx, &prac, hands, base)
				if err != nil {
					tx.Rollback()
					return err
				}
			}
		}
	}

	err = tx.Commit()
	return err
}

var counter = 0

func GenScale(tx timelearn.Populator, prac *practice, hands, base string) error {
	numHands := 1
	if hands == "2H" {
		numHands = 2
	}

	style, ok := scaleStyles[prac.style]
	if !ok {
		panic("Invalid style")
	}

	notes, err := MakeScale(base, &Scale{
		Intervals: prac.intervals,
		Pattern:   style.pattern,
		Octaves:   style.octaves,
		Extra:     style.extra,
		Hands:     numHands,
	})
	if err != nil {
		return err
	}

	voice := Voicing{
		Chords: notes,
		Type:   "voicing",
	}

	vtext, err := json.Marshal(&voice)
	if err != nil {
		return err
	}

	counter++
	qn := fmt.Sprintf("%s-scale %s %s", hands, base, prac.name)
	err = tx.Add(qn, string(vtext))
	return err
}

// The order that the keys are practiced in.  We duplicate F♯ and G♭
// to make sure resulting patterns are recognized in both.
var allKeys = []string{"C", "G", "D", "A", "E", "B", "F♯", "G♭", "D♭", "A♭", "E♭", "B♭", "F"}

type practice struct {
	name      string
	intervals string
	hands     []string
	style     string
}

// This is the table of exercises.  Keeping the ordering consistent
// (so the same problem id is generated) will allow the tool to update
// an existing database.
var scalePractice = []practice{
	{
		name:      "major",
		intervals: "WWHWWWH",
		hands:     bothHandsProgressive,
		style:     "updown",
	},
	{
		name:      "minor (dorian)",
		intervals: "WHWWWHW",
		hands:     bothHands,
		style:     "updown",
	},
	{
		name:      "dominant (mixolydian)",
		intervals: "WWHWWHW",
		hands:     bothHands,
		style:     "updown",
	},
	{
		name:      "half diminished (locrian)",
		intervals: "HWWHWWW",
		hands:     bothHands,
		style:     "updown",
	},
	{
		name:      "diminished (whole-half)",
		intervals: "WHWHWHWH",
		hands:     bothHandsProgressive,
		style:     "updown",
	},
	{
		name:      "sym-dom (half-whole)",
		intervals: "HWHWHWHW",
		hands:     bothHands,
		style:     "updown",
	},

	{
		name:      "major 3rds",
		intervals: "WWHWWWH",
		hands:     bothHandsProgressive,
		style:     "3up",
	},
	{
		name:      "major 3rds rev",
		intervals: "WWHWWWH",
		hands:     bothHandsProgressive,
		style:     "3upr",
	},
}

var (
	bothHandsProgressive = []string{"RH", "LH", "2H"}
	bothHands            = []string{"2H"}
	scaleStyles          = map[string]struct {
		pattern []int
		octaves int
		extra   int
	}{
		"updown": {
			pattern: []int{0},
			octaves: 2,
			extra:   0,
		},
		"3up": {
			pattern: []int{0, 2},
			octaves: 1,
			extra:   2,
		},
		"3upr": {
			pattern: []int{2, 0},
			octaves: 1,
			extra:   2,
		},
	}
)
