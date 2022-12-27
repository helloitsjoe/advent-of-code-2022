package advent08

import (
	"fmt"
	"log"
	"os"
	"path"
	"strconv"
	"strings"
)

func FindInvisibleTrees(content []string) []string {
	invisibleFromLeft := map[string]bool{}
	invisibleFromRight := map[string]bool{}
	// invisibleFromTop := map[string]bool{}
	// invisibleFromBottom := map[string]bool{}

	invisible := []string{}

	highestLeft := 0
	highestRight := 0

	x := 0
	y := 0
	for y < len(content) {
		for x < len(content[y]) {
			currLeft, _ := strconv.Atoi(string(content[y][x]))
			currRight, _ := strconv.Atoi(string(content[y][len(content[y])-1-x]))

			if currLeft >= highestLeft {
				currLeft = highestLeft
			} else {
				invisibleFromLeft[string(x)+":"+string(y)] = true
			}

			if currRight >= highestRight {
				currRight = highestRight
			} else {
				invisibleFromRight[string(len(content[y])-1-x)+":"+string(y)] = true
			}

			x += 1
		}
		y += 1
	}

	for k, _ := range invisibleFromLeft {
		if invisibleFromRight[k] == true {
			invisible = append(invisible, k)
		}
	}

	return invisible
}

func Run() {
	filePath := path.Join("..", "advent08", "input.txt")
	bytes, err := os.ReadFile(filePath)
	if err != nil {
		log.Fatal("Error reading file")
	}
	content := strings.Split(strings.TrimSpace(string(bytes)), "\n")

	invisibleCoords := FindInvisibleTrees(content)
	fmt.Println(invisibleCoords)
}
