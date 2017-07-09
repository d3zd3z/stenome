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
    ["Dm7", "G7", "CΔ"],
    ["Am7", "D7", "GΔ"],
    ["Em7", "A7", "DΔ"],
    ["Bm7", "E7", "AΔ"],
    ["F♯m7", "B7", "EΔ"],
    ["C♯m7", "F♯7", "BΔ"],
    ["G♯m7", "C♯7", "F♯Δ"],
    ["A♭m7", "D♭7", "G♭Δ"],
    ["E♭m7", "A♭7", "D♭Δ"],
    ["B♭m7", "E♭7", "A♭Δ"],
    ["Fm7", "B♭7", "E♭Δ"],
    ["Cm7", "F7", "B♭Δ"],
    ["Gm7", "C7", "FΔ"] ]

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

println("DELETE FROM probs;")

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
