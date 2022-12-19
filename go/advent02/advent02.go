package advent02

import (
	"fmt"
	"log"
	"os"
	"path/filepath"
	"strings"
)

func ComputeScore(a string, b string) int {
	// Values
	// A = X = 1 = Rock
	// B = Y = 2 = Paper
	// C = Z = 3 = Scissors
	// Win = 6
	// Draw = 3
	// Lose = 0

	values := map[string]int{
		"A": 1,
		"B": 2,
		"C": 3,
		"X": 1,
		"Y": 2,
		"Z": 3,
	}

	valA := values[a]
	valB := values[b]

	score := valB
	if valA == valB {
		score += 3
	} else if valB == valA+1 || valB == valA-2 {
		score += 6
	}

	return score
}

func Run() {
	path := filepath.Join("..", "advent02", "input.txt")
	file, err := os.ReadFile(path)

	if err != nil {
		log.Fatalf("Error reading file: %v", err)
	}

	f := strings.Split(string(file), "\n")
	l := f[:len(f)-1]
	total := 0
	for _, line := range l {
		scores := strings.Split(line, " ")
		score := ComputeScore(scores[0], scores[1])
		total += score
	}

	fmt.Println(total)
}
