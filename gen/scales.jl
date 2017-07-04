# Generate scales.

# This ID is incremented for every problem generated.  Barring changes
# to the generation, this should generate the same ids on subsequent
# runs.
cur_id = 1

function emit(question, answer)
    global cur_id
    println("INSERT INTO \"probs\" VALUES($cur_id, '$question', '$answer');")
    cur_id += 1
end

key_names = [ "C", "G", "D", "A", "E", "B", "F♯", "G♭", "D♭", "A♭", "E♭", "B♭", "F" ]

function gen_scale(name, hand)
    for key in key_names
        emit("$hand-scale $key $name", "play")
    end
end

function gen_scale(name)
    gen_scale(name, "RH")
    gen_scale(name, "LH")
    gen_scale(name, "2H")
end

gen_scale("major")
gen_scale("minor (dorian)")
gen_scale("dominant (mixolydian)")
gen_scale("half diminished (locrian)")
gen_scale("diminished (whole-half)")
