package gen

import (
	"fmt"
)

type Voicing struct {
	Chords [][]Note `json:"chords"`
	Type   string   `json:"type"`
}

type Note int

// The description of a scale exercise.  This describes how to build a
// scale exercise.
type Scale struct {
	Intervals string // Intervals between the notes, must describe 1 octave.
	Pattern   []int  // The pattern for arps.
	Octaves   int    // How many octaves for this pattern.
	Extra     int    // How many extra patterns to go below start at end.
	Hands     int    // How many hands to play
}

// Generate a scale starting at a given base note, using the given
// intervals (which should cover exactly an octave), and the given
// pattern ({0} would just be the scale up and down, {0,2} would be a
// 3-up pattern, for example).
func MakeScale(base string, scale *Scale) ([][]Note, error) {
	b, err := NoteOfText(base)
	if err != nil {
		return nil, err
	}

	b -= 12

	first := b
	last := b
	source := []Note{b}

	// Fill in the source with enough notes to cover an octave
	// before and after the exercise.
	for i := 0; i < scale.Octaves+2; i++ {
		for _, ch := range scale.Intervals {
			next := Note(0)
			switch ch {
			case 'H':
				next = 1
			case 'W':
				next = 2
			case 'm':
				next = 3
			case 'M':
				next = 4
			case '5':
				next = 5
			default:
				return nil, fmt.Errorf("Invalid interval char '%c'", ch)
			}

			next += last
			source = append(source, next)
			last = next
		}
		if last != first+12 {
			return nil, fmt.Errorf("Scale is not exact octave: %q", scale.Intervals)
		}
		first = last
	}

	perOctave := len(scale.Intervals) // Valid because above would die if non-ascii.

	var notes []Note

	// Build the patterns going up.
	for n := perOctave; n < perOctave*(scale.Octaves+1); n++ {
		for _, p := range scale.Pattern {
			notes = append(notes, source[n+p])
		}
	}

	// And then back down.
	for n := perOctave * (scale.Octaves + 1); n > perOctave-scale.Extra; n-- {
		for _, p := range scale.Pattern {
			notes = append(notes, source[n+p])
		}
	}

	// And end with the final note.
	notes = append(notes, source[perOctave])

	return ExpandNotes(notes, scale.Hands), nil
}

// Explode a sequence of notes into "chords" of those notes that play
// each as a single note.  The number of hands sets how many hands
// should be playing the pattern.
func ExpandNotes(seq []Note, hands int) [][]Note {
	result := make([][]Note, 0, len(seq))

	for _, ch := range seq {
		notes := make([]Note, 0, hands)
		for i := 0; i < hands; i++ {
			notes = append(notes, Note(int(ch)+12*i))
		}

		result = append(result, notes)
	}

	return result
}

// Convert a midi note value into a name/octave, with C0 being the
// lowest midi note.
func (n Note) String() string {
	oct := 0
	for n > 11 {
		oct++
		n -= 12
	}
	return fmt.Sprintf("%s%d", names[n], oct)
}

var names = []string{
	"C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
}

// Decode a textual representation of a note.  Notes are a letter
// possibly followed by a sharp or flat sign (in ascii or Unicode).
// The resulting note will be between middle C and the C above that
// (for a B#).  Returns an error if the not is not valid.
func NoteOfText(text string) (Note, error) {
	base := -1
	acc := 3

	for _, ch := range text {
		nbase, ok := notes[ch]
		if ok {
			if base != -1 {
				return 0, fmt.Errorf("Invalid note: %q", text)
			}
			base = nbase
			continue
		}

		nacc, ok := accidentals[ch]
		if ok {
			if acc != 3 {
				return 0, fmt.Errorf("Invalid note: %q", text)
			}
			acc = nacc
			continue
		}

		return 0, fmt.Errorf("Invalid note: %q", text)
	}

	if acc == 3 {
		acc = 0
	}

	if base == -1 {
		return 0, fmt.Errorf("Invalid note: %q", text)
	}

	return Note(base + acc), nil
}

var (
	notes = map[rune]int{
		'C': 60,
		'D': 62,
		'E': 64,
		'F': 65,
		'G': 67,
		'A': 69,
		'B': 71,
	}
	accidentals = map[rune]int{
		'#': 1,
		'♯': 1,
		'b': -1,
		'♭': -1,
	}
)
