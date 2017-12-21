package midilearn

// Modify the user nodes by zero or more octaves to possibly match the
// given input notes.  Returns a true if the first note/chord could be
// matched, otherwise false indicates that the first note wasn't
// right.
func adjustOctave(user, expect [][]Note) bool {
	// Unsure if these checks should be here, or just get the
	// panic from invalid dereferencing.
	/*
		if len(expect) == 0 {
			panic("Expect shouldn't be empty")
		}

		if len(user) == 0 {
			panic("User input shouldn't be empty")
		}
	*/

	u := user[0]
	e := expect[0]

	if u[0] == e[0] {
		return true
	}

	shift := int(u[0]) - int(e[0])
	if shift == 0 {
		return true
	}
	if shift%12 == 0 {
		for a := range user {
			for b := range user[a] {
				user[a][b] = Note(int(user[a][b]) - shift)
			}
		}
		return true
	}

	return false
}
