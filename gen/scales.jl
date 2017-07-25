# Generate scales.

import JSON

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

function gen_scale(ival, hands, name, hand)
    for key in key_names
        answer = Dict("type" => "scale",
            "base" => key,
            "intervals" => ival,
            "hands" => hands,
            "octaves" => 2,
            "style" => "updown")
        emit("$hand-scale $key $name", JSON.json(answer))
    end
end

function gen_scale(ival, name)
    gen_scale(ival, 1, name, "RH")
    gen_scale(ival, 1, name, "LH")
    gen_scale(ival, 2, name, "2H")
end

println("DELETE FROM probs;")
# Learn major scales with both hands.
gen_scale("WWHWWWH", "major")

# The rest of the scales, just right hand.
gen_scale("WHWWWHW", 1, "minor (dorian)", "RH")
gen_scale("WWHWWHW", 1, "dominant (mixolydian)", "RH")
gen_scale("HWWHWWW", 1, "half diminished (locrian)", "RH")
gen_scale("WHWHWHWH", 1, "diminished (whole-half)", "RH")
