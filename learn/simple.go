package learn

import (
	"fmt"
	"os"

	"davidb.org/x/stenome/timelearn"
	"golang.org/x/crypto/ssh/terminal"
)

type simpleUI struct{}

func newSimpleUI() (UI, error) {
	return &simpleUI{}, nil
}

func (u *simpleUI) Close() error {
	return nil
}

func (u *simpleUI) Single(prob, next *timelearn.Problem) (int, error) {
	fmt.Printf("Q: %s: ", prob.Question)

	defer fmt.Printf("\n")

	// As a special case, if the answer is just the string "play",
	// don't wait for space and an answer.
	if prob.Answer != "play" && prob.Answer != "played" {
		for {
			ch, err := u.readChar()
			if err != nil {
				return 0, err
			}

			if ch == 27 {
				// Zero is the marker to stop.
				return 0, nil
			}
			if ch == ' ' {
				break
			}
		}
		fmt.Printf("\n\nA: %s (1=bad, 4=good): ", prob.Answer)
	} else {
		fmt.Printf("\n\n    (1=bad, 4=good): ")
	}

	for {
		ch, err := u.readChar()
		if err != nil {
			return 0, nil
		}

		if ch == 27 {
			return 0, nil
		}

		if ch >= '1' && ch <= '4' {
			return (ch - '0'), nil
		}
	}
}

// Get a single keypress back from the user, without waiting.
func (u *simpleUI) readChar() (int, error) {
	old, err := terminal.MakeRaw(0)
	if err != nil {
		return 0, err
	}
	defer terminal.Restore(0, old)

	buf := make([]byte, 1)
	_, err = os.Stdin.Read(buf)
	return int(buf[0]), err
}
