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

# Diminished tri-tone sub.  The idea of this pattern is to alternate
# the inversion of the chords so that the pattern only moves slightly
# on the piano.  See p39 in the deGreg book for an example.
min_tritone_sub = [
    ["CΔ", "C♯°7", "Dm7", "D♯°7", "Em7", "E♭7", "Dm7", "D♭7", "CΔ"],
    ["FΔ", "F♯°7", "Gm7", "G♯°7", "Am7", "A♭7", "Gm7", "G♭7", "FΔ"],
    ["B♭Δ", "B°7", "Cm7", "C♯°7", "Dm7", "D♭7", "Cm7", "B7", "B♭Δ"],
    ["E♭Δ", "E°7", "Fm7", "F♯°7", "Gm7", "G♭7", "Fm7", "E7", "E♭Δ"],

    ["A♭Δ", "A°7", "B♭m7", "B°7", "Cm7", "B7", "B♭m7", "A7", "A♭Δ"],
    ["D♭Δ", "D°7", "E♭m7", "E°7", "Fm7", "E7", "E♭m7", "D7", "D♭Δ"],
    ["G♭Δ", "G°7", "A♭m7", "A°7", "B♭m7", "A7", "A♭m7", "G7", "G♭Δ"],
    ["BΔ", "C°7", "C♯m7", "D°7", "D♯m7", "D7", "C♯m7", "C7", "BΔ"],

    ["EΔ", "F°7", "F♯m7", "G°7", "G♯m7", "G7", "F♯m7", "F7", "EΔ"],
    ["AΔ", "A♯°7", "Bm7", "C°7", "C♯m7", "C7", "Bm7", "B♭7", "AΔ"],
    ["DΔ", "D♯°7", "Em7", "F°7", "F♯m7", "F7", "Em7", "E♭7", "DΔ"],
    ["GΔ", "G♯°7", "Am7", "A♯°7", "Bm7", "B♭7", "Am7", "A♭7", "GΔ"] ]

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

# Exercise 2-1A, ii-V-I, guide tones, 1-7-3 first.
gen_cycle("2-1A", major_iivi, "guide", "1-7-3")

# Exercise 2-1B, ii-V-I, guide tones, 1-3-7 first.
gen_cycle("2-1B", major_iivi, "guide", "1-3-7")

# Exercise 2-2A, I, guide tones, 1-7-3 first.
gen_cycle("2-1A", major_iivi, "guide", "1-7-3", chord_start=2)

# Exercise 2-2B, I, guide tones, 1-3-7 first.
gen_cycle("2-1B", major_iivi, "guide", "1-3-7", chord_start=2)

# Exercise 2-3A, V-I, guide tones, 1-7-3 first.
gen_cycle("2-1A", major_iivi, "guide", "1-7-3", chord_start=3)

# Exercise 2-3B, V-I, guide tones, 1-3-7 first.
gen_cycle("2-1B", major_iivi, "guide", "1-3-7", chord_start=3)

# Exercise 2-4A, Diminshed / Tri-tone sub, guide tones, 1-7-3 first.
gen_cycle("2-4A", min_tritone_sub, "guide", "1-7-3")

# Exercise 2-4B, Diminshed / Tri-tone sub, guide tones, 1-3-7 first.
gen_cycle("2-4B", min_tritone_sub, "guide", "1-3-7")
