// Learn using midi

package midilearn // import "davidb.org/x/stenome/midilearn"

import (
	"encoding/json"
	"fmt"
	"log"
	"sort"
	"time"

	"davidb.org/x/stenome/timelearn"
	"github.com/rakyll/portmidi"
	"github.com/texttheater/golang-levenshtein/levenshtein"
)

type Midi struct {
	in *portmidi.Stream
}

func NewMidi() (*Midi, error) {
	portmidi.Initialize()
	in, err := portmidi.NewInputStream(portmidi.DefaultInputDeviceID(), 1024)
	if err != nil {
		return nil, err
	}

	return &Midi{in: in}, nil
}

func (m *Midi) Close() error {
	return m.in.Close()
}

func (m *Midi) Single(prob, next *timelearn.Problem) (int, error) {
	st1, err := m.singleOnce(prob, next)
	if err != nil {
		return 0, err
	}
	if st1 == 0 {
		return 0, nil
	}

	stn := st1
	for {
		// If good, return the initial status.
		if stn == 4 {
			return st1, nil
		}

		// If stop requested, return that.
		if stn == 0 {
			return 0, nil
		}

		fmt.Printf("** Mistakes were, please play again **\n")
		stn, err = m.singleOnce(prob, next)
		if err != nil {
			return 0, err
		}
	}
}

// Ask the user once to play.
func (m *Midi) singleOnce(prob, next *timelearn.Problem) (int, error) {
	fmt.Printf("Play: %s\n", prob.Question)
	if next != nil {
		fmt.Printf("      %s\n", next.Question)
	}

	var chords Chords
	err := json.Unmarshal([]byte(prob.Answer), &chords)
	if err != nil {
		return 0, err
	}
	// fmt.Printf("%v\n", chords)

	err = m.Drain()
	if err != nil {
		return 0, err
	}

	user, err := m.Record(6)
	if err != nil {
		return 0, err
	}

	if adjustOctave(user, chords.Chords) {
		// The levenshtein distance uses []rune, so convert to
		// a possibly meaningless, but working rune sequence
		// for comparison.
		uu := runify(user)
		ee := runify(chords.Chords)

		// fmt.Printf("Compare:\n   %q\nto %q\n", uu, ee)

		dist := levenshtein.DistanceForStrings(ee, uu, levenshtein.DefaultOptions)
		fmt.Printf("There are %d differences", dist)
		if dist <= 3 {
			return (4 - dist), nil
		}

		return 1, nil
	} else {
		fmt.Printf("First note mismatch, stopping\n")
		return 0, nil
	}

	return 0, nil
}

// Midi notes can take values from 0 to 127.  Pack the notes into a
// slice of runes with separators using an invalid midi note value.
// The extra separator doesn't hurt anything.
func runify(notes [][]Note) []rune {
	var result []rune
	for _, ch := range notes {
		result = append(result, 256)

		for _, n := range ch {
			result = append(result, rune(n))
		}
	}

	return result
}

// Drain any queued midi events.
func (m *Midi) Drain() error {
	for {
		events, err := m.in.Read(1024)
		if err != nil {
			return err
		}
		if len(events) == 0 {
			break
		}
	}

	return nil
}

// Record the user playing.  Consider the recording done after
// timeout*250ms of no note down events.
func (m *Midi) Record(timeout int) ([][]Note, error) {
	var cap capture

	for {
		events, err := m.in.Read(1024)
		if err != nil {
			return nil, err
		}

		if len(events) == 0 {
			cap.addIdle()
			if cap.idleCount >= timeout {
				break
			}
			time.Sleep(250 * time.Millisecond)
			continue
		}

		for i := range events {
			cap.addEvent(&events[i])
		}
	}

	// Sort each note with the chords, as we don't care about the
	// precise order the notes within a chord are played.
	for i := range cap.notes {
		sort.Sort(NoteSlice(cap.notes[i]))
	}

	return cap.notes, nil
}

// A Capture keeps track of the state of capture of midi data.
type capture struct {
	notes     [][]Note
	idleCount int
	lastTime  portmidi.Timestamp
}

// Add a midi event to the captured data.
func (c *capture) addEvent(ev *portmidi.Event) {
	// Only consider note down events.
	if ev.Status&0xf0 != 0x90 {
		return
	}

	// If the last note(s) played are recent enough, add this note
	// as a chord.  80ms is fairly arbitrarily chosen, but seems
	// to work pretty well.
	if len(c.notes) > 0 && ev.Timestamp-c.lastTime < 80 {
		last := len(c.notes) - 1
		c.notes[last] = append(c.notes[last], Note(ev.Data1))
	} else {
		// Otherwise, this note starts a new chord.
		c.notes = append(c.notes, []Note{Note(ev.Data1)})
	}
	c.lastTime = ev.Timestamp
	c.idleCount = 0
}

// Indicate no notes came in, returns true if it is time to stop
func (c *capture) addIdle() {
	c.idleCount++
	if len(c.notes) == 0 {
		c.idleCount = 0
	}
}

type Chords struct {
	Type   string   `json:"type"`
	Chords [][]Note `json:"chords"`
}

type Note uint8

type NoteSlice []Note

func (p NoteSlice) Len() int           { return len(p) }
func (p NoteSlice) Less(i, j int) bool { return p[i] < p[j] }
func (p NoteSlice) Swap(i, j int)      { p[i], p[j] = p[j], p[i] }
