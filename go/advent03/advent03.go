package advent03

import (
	"fmt"
	"log"
	"os"
	"path"
	"strings"
)

// Read file
// Split on newlines
// Split each line in half
// Find duplicate char
// Sum priorities of all duplicates
// Priority = a-z 1-26 A-Z 27-52

func convertRuneToPriority(r rune) int {
	if r > 96 {
		return int(r - 96)
	}

	return int(r - 64 + 26)
}

func Run() {
	path := path.Join("..", "advent03", "input.txt")
	file, err := os.ReadFile(path)

	if err != nil {
		log.Fatalf("Error reading file: %v", err)
	}

	trimmed := strings.TrimSpace(string(file))
	lines := strings.Split(trimmed, "\n")

	total := 0
	for _, line := range lines {
		// split in half
		head := line[0 : len(line)/2]
		tail := line[len(line)/2:]

		headMap := map[rune]bool{}
		for _, c := range head {
			headMap[c] = true
		}

		// find duplicate
		for _, c := range tail {
			if headMap[c] == true {
				priority := convertRuneToPriority(c)
				// add priority to sum
				total += priority
				break
			}
		}
	}

	fmt.Println(total)
}
