package main

import (
	"fmt"
	"os"

	"davidb.org/x/stenome/gen"
	"davidb.org/x/stenome/learn"
	"davidb.org/x/stenome/timelearn"
)

func main() {
	if len(os.Args) != 3 {
		fmt.Printf("Usage: stenome [run|gen] lesson.db\n")
		return
	}

	if os.Args[1] == "gen" {
		err := gen.GenScales()
		if err != nil {
			fmt.Printf("Error generating scales: %q\n", err)
			return
		}

		return
	}

	if os.Args[1] != "run" {
		fmt.Printf("Unknown command\n")
		return
	}

	tl, err := timelearn.Open(os.Args[2])
	if err != nil {
		fmt.Printf("Error with database: %q\n", err)
		return
	}

	err = learn.Drill(tl)
	if err != nil {
		fmt.Printf("Error learning: %q\n", err)
	}
}
