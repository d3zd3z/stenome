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

function gen_problem(desc)
    for key in key_names
        emit("$key $desc", "played")
    end
end

println("DELETE FROM probs;")

# Learn symmetric dominant over ii-V-I major.
gen_problem("sym dom over ii-V-I")
