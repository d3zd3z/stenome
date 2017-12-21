// +build midi

package main

import (
	"davidb.org/x/stenome/learn"
	"davidb.org/x/stenome/midilearn"
)

// If midi is tagged in the build, add support for it.
func init() {
	learn.Register("midi", func() (learn.UI, error) {
		return midilearn.NewMidi()
	})
}
