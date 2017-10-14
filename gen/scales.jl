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

function gen_scale(ival, hands, name, hand; style="updown", octaves=2)
    for key in key_names
        answer = Dict("type" => "scale",
            "base" => key,
            "intervals" => ival,
            "hands" => hands,
            "octaves" => octaves,
            "style" => style)
        emit("$hand-scale $key $name", JSON.json(answer))
    end
end

function gen_scale(ival, name; style="updown", octaves=2)
    gen_scale(ival, 1, name, "RH", style=style, octaves=octaves)
    gen_scale(ival, 1, name, "LH", style=style, octaves=octaves)
    gen_scale(ival, 2, name, "2H", style=style, octaves=octaves)
end

println("DELETE FROM probs;")
# Learn major scales with both hands.
gen_scale("WWHWWWH", "major")

# The rest of the scales, just right hand.
gen_scale("WHWWWHW", 2, "minor (dorian)", "2H")
gen_scale("WWHWWHW", 2, "dominant (mixolydian)", "2H")
gen_scale("HWWHWWW", 2, "half diminished (locrian)", "2H")
#gen_scale("WHWHWHWH", 2, "diminished (whole-half)", "2H")
gen_scale("WHWHWHWH", "diminished (whole-half)")
gen_scale("HWHWHWHW", 2, "sym-dom (half-whole)", "2H")

# Simple patterns on major.
gen_scale("WWHWWWH", 1, "major 3rds", "RH", style="3up",
    octaves=1)
gen_scale("WWHWWWH", 1, "major 3rds", "LH", style="3up",
    octaves=1)
gen_scale("WWHWWWH", 2, "major 3rds", "2H", style="3up",
    octaves=1)
gen_scale("WWHWWWH", 1, "major 3rds rev", "RH", style="3upr",
    octaves=1)
gen_scale("WWHWWWH", 1, "major 3rds rev", "LH", style="3upr",
    octaves=1)
gen_scale("WWHWWWH", 2, "major 3rds rev", "2H", style="3upr",
    octaves=1)
