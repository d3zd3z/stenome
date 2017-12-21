// Learn using midi

package main

import (
	"fmt"
	"log"
	"sort"
	"time"

	"github.com/rakyll/portmidi"
)

type Midi struct {
	in *portmidi.Stream
}

func main() {
	fmt.Printf("midi\n")
	portmidi.Initialize()
	fmt.Printf("There are %d devices\n", portmidi.CountDevices())
	fmt.Printf("Default input: %d\n", portmidi.DefaultInputDeviceID())

	for i := 0; i < portmidi.CountDevices(); i++ {
		fmt.Printf("[%d]: %+v\n", i, portmidi.Info(portmidi.DeviceID(i)))
	}

	in, err := portmidi.NewInputStream(portmidi.DefaultInputDeviceID(), 1024)
	if err != nil {
		log.Fatal(err)
	}
	defer in.Close()

	m := Midi{in: in}

	err = m.Drain()
	if err != nil {
		log.Fatal(err)
	}

	fmt.Printf("Start playing\n")

	notes, err := m.Record(8)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Printf("Notes: %+v", notes)
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

type Note uint8

type NoteSlice []Note

func (p NoteSlice) Len() int           { return len(p) }
func (p NoteSlice) Less(i, j int) bool { return p[i] < p[j] }
func (p NoteSlice) Swap(i, j int)      { p[i], p[j] = p[j], p[i] }
