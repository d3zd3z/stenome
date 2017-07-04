# Generate voicings exercises.

# This ID is incremented for every problem generated.  Barring changes
# to the generation, this should generate the same ids on subsequent
# runs.
cur_id = 1

function emit(question, answer)
    global cur_id
    println("INSERT INTO \"probs\" VALUES($cur_id, '$question', '$answer');")
    cur_id += 1
end

# The ii-V-I progressions through the cycle.  The DeGregg puts some
# work into keeping these in range.  However, since we will be mostly
# doing these out of order, the exact order here doesn't matter all
# that much, and the simple Circle of V is fine.  F♯ and G♭ and both
# printed (making for 13 of each voicing), since both are common.
major_iivi = [
    ["Dm7", "G7", "CM7"],
    ["Am7", "D7", "GM7"],
    ["Em7", "A7", "DM7"],
    ["Bm7", "E7", "AM7"],
    ["F♯m7", "B7", "EM7"],
    ["C♯m7", "F♯7", "BM7"],
    ["G♯m7", "C♯7", "F♯M7"],
    ["A♭m7", "D♭7", "G♭M7"],
    ["E♭m7", "A♭7", "D♭M7"],
    ["B♭m7", "E♭7", "A♭M7"],
    ["Fm7", "B♭7", "E♭M7"],
    ["Cm7", "F7", "B♭M7"],
    ["Gm7", "C7", "FM7"] ]

# Generate patterns for a particular voicing.  The name will be
# printed before, and the hint afterwards.  This should be enough for
# the reader to understand how to voice the chord.  The chord start
# can be used to skip part of the progression.
function gen_cycle(exercise, progression, name, hint; chord_start=1)
    for prog in progression
        chord = join(prog[chord_start:end], "-")
        emit("$name $chord $hint ($exercise)", "play")
    end
end

# Exercise 1-1A, ii-V-I, voiced with a 1-7 shell first.
gen_cycle("1-1A", major_iivi, "shell", "1-7")

# Exercise 1-1B, ii-V-I, voiced with a 1-3 shell first.
gen_cycle("1-1B", major_iivi, "shell", "1-3")

# Exercise 1-1A, ii-V-I, voiced with a 1-7 shell first.
gen_cycle("1-2A", major_iivi, "shell", "1-3", chord_start=2)

# Exercise 1-1B, ii-V-I, voiced with a 1-3 shell first.
gen_cycle("1-2B", major_iivi, "shell", "1-7", chord_start=2)

# Exercise 1-1A, ii-V-I, voiced with a 1-7 shell first.
gen_cycle("1-3A", major_iivi, "shell", "1-7", chord_start=3)

# Exercise 1-1B, ii-V-I, voiced with a 1-3 shell first.
gen_cycle("1-3B", major_iivi, "shell", "1-3", chord_start=3)
