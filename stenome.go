package main

import (
	"fmt"
	"os"

	"davidb.org/x/stenome/learn"
	"davidb.org/x/stenome/timelearn"
)

func main() {
	if len(os.Args) != 2 {
		fmt.Printf("Usage: stenome lesson.db\n")
		return
	}

	tl, err := timelearn.Open(os.Args[1])
	if err != nil {
		fmt.Printf("Error with database: %q\n", err)
		return
	}

	err = learn.Drill(tl)
	if err != nil {
		fmt.Printf("Error learning: %q\n", err)
	}
}
