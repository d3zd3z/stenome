package gen_test

import (
	"fmt"
	"testing"

	"davidb.org/x/stenome/gen"
)

func TestNotes(t *testing.T) {
	for _, text := range badNotes {
		_, err := gen.NoteOfText(text)
		if err == nil {
			t.Fatalf("Failure to detect invalid note: %q", text)
		}
	}

	for _, gn := range goodNotes {
		got, err := gen.NoteOfText(gn.text)
		if err != nil {
			t.Fatal(err)
		}
		if got != gn.expect {
			t.Fatalf("Incorrect parse of note %q, got %d, expect %d", gn.text, got, gn.expect)
		}
	}

}

var badNotes = []string{
	"C$",
	"C##",
	"C#b",
	"Cbb",
	"C♭♯",
}

var goodNotes = []struct {
	text   string
	expect gen.Note
}{
	{"C", 60},
	{"Cb", 59},
	{"C#", 61},
	{"C♭", 59},
	{"C♯", 61},
	{"D", 62},
	{"Db", 61},
	{"D#", 63},
	{"D♭", 61},
	{"D♯", 63},
	{"E", 64},
	{"F", 65},
	{"G", 67},
	{"A", 69},
	{"B", 71},
	{"B#", 72},
}

func TestScale(t *testing.T) {
	expect := "[[C5] [D5] [E5] [F5] [G5] [A5] [B5] " +
		"[C6] [D6] [E6] [F6] [G6] [A6] [B6] " +
		"[C7] [B6] [A6] [G6] [F6] [E6] [D6] " +
		"[C6] [B5] [A5] [G5] [F5] [E5] [D5] [C5]]"
	seq, err := gen.MakeScale("C", &gen.Scale{
		Intervals: "WWHWWWH",
		Pattern:   []int{0},
		Octaves:   2,
		Extra:     0,
		Hands:     1,
	})
	if err != nil {
		t.Fatal(err)
	}
	text := fmt.Sprintf("%v", seq)
	if text != expect {
		t.Fatalf("Incorrect result %q != %q", text, expect)
	}

	expect = "[[F#5 F#6] [A#5 A#6] [G#5 G#6] [B5 B6] [A#5 A#6] [C#6 C#7] [B5 B6] [D#6 D#7] " +
		"[C#6 C#7] [F6 F7] [D#6 D#7] [F#6 F#7] [F6 F7] [G#6 G#7] [F#6 F#7] [A#6 A#7] " +
		"[F6 F7] [G#6 G#7] [D#6 D#7] [F#6 F#7] [C#6 C#7] [F6 F7] [B5 B6] [D#6 D#7] " +
		"[A#5 A#6] [C#6 C#7] [G#5 G#6] [B5 B6] [F#5 F#6] [A#5 A#6] [F5 F6] [G#5 G#6] [F#5 F#6]]"
	seq, err = gen.MakeScale("F#", &gen.Scale{
		Intervals: "WWHWWWH",
		Pattern:   []int{0, 2},
		Octaves:   1,
		Extra:     2,
		Hands:     2,
	})
	if err != nil {
		t.Fatal(err)
	}
	text = fmt.Sprintf("%v", seq)
	if text != expect {
		t.Fatal("Incorrect result %q != %q", text, expect)
	}
}
