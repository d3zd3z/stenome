# Generate voicings exercises.

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

type Transpose
    name :: String
    amount :: Int
end

# Transpositions for licks.  The order is mixed up a bit.
transposes = [
    Transpose("C", 0),
    Transpose("G", 7),
    Transpose("F", 5),
    Transpose("D", 2),
    Transpose("B♭", 10),
    Transpose("A", 9),
    Transpose("E♭", 3),
    Transpose("E", 4),
    Transpose("B", 11),
    Transpose("A♭", 8),
    Transpose("F♯", 6),
    Transpose("C♯", 1),
    Transpose("G♭", 6) ]

function transpose(notes :: Vector{Vector{Int}}, by :: Transpose)
    map(notes) do chord
        chord .+ by.amount
    end
end

function make_notes(notes :: Vector{Vector{Int}}, by :: Transpose)
    Dict("type" => "lick",
        "notes" => transpose(notes, by))
end

# Generate patterns for a particular voicing.  The name will be
# printed before, and the hint afterwards.  This should be enough for
# the reader to understand how to voice the chord.  The chord start
# can be used to skip part of the progression.
function gen_cycle(name, notes)
    for by in transposes
        emit("$name in $(by.name)", JSON.json(make_notes(notes, by)))
    end
end

println("DELETE FROM probs;")

# Jaki Byard, lick 1
gen_cycle("Byard 1", 
    [ [60],[62],[64],[67],[64,65],[67],[63,64],[60],[57],[60] ])
gen_cycle("Byard 2",
    [ [50], [53], [57], [60], [59], [62], [65], [67], [68], [70], [71], [74], [72], [67] ])
gen_cycle("Martin 1",
    [ [63], [64], [69], [67], [64], [60], [65], [63], [64], [60],
      [57], [56], [55], [55] ])
gen_cycle("Martin 2",
    map([ [63],[64],[67],[70],[73],[75],[72] ]) do chord
        chord .- 5
    end)
