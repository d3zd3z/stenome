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

function gen_scale(ival, name, hand)
    for key in key_names
        answer = Dict("type" => "scale",
            "base" => key,
            "intervals" => ival,
            "hands" => 1,
            "octaves" => 2,
            "style" => "updown")
        emit("$hand-scale $key $name", JSON.json(answer))
    end
end

function gen_scale(ival, name)
    gen_scale(ival, name, "RH")
    gen_scale(ival, name, "LH")
    gen_scale(ival, name, "2H")
end

gen_scale("WWHWWWH", "major")
gen_scale("WHWWWHW", "minor (dorian)")
gen_scale("WWHWWHW", "dominant (mixolydian)")
gen_scale("HWWHWWW", "half diminished (locrian)")
gen_scale("WHWHWHWH", "diminished (whole-half)")
