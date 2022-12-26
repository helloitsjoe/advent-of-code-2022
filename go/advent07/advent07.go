package advent07

import (
	"fmt"
	"log"
	"os"
	"path"
	"strconv"
	"strings"
)

type Dir = map[string]int

func BuildTree(content []string) Dir {
	dirs := Dir{}
	currDir := "/"

	for _, line := range content {
		if line[0] == '$' {
			if strings.Contains(line, "$ cd") {
				target := strings.Split(line, "$ cd ")[1]
				childTotal := dirs[currDir]
				currDir = path.Join(currDir, target)
				// Cheeky! Will only add to total if we move to the parent
				if target == ".." {
					dirs[currDir] += childTotal
				}
			}
		} else {
			temp := strings.Split(line, " ")
			// Unused name for now, might need in part two?
			idOrSize, _ := temp[0], temp[1]
			if idOrSize == "dir" {
				// noop
			} else {
				size, _ := strconv.Atoi(idOrSize)
				dirs[currDir] += size
			}
		}
	}

	return dirs
}

func Run() {
	inputPath := path.Join("..", "advent07", "input.txt")
	file, err := os.ReadFile(inputPath)
	if err != nil {
		log.Fatal("Error reading file")
	}
	content := string(file)

	tree := BuildTree(strings.Split(strings.TrimSpace(content), "\n"))

	total := 0
	for k, v := range tree {
		if v < 100000 {
			total += v
			fmt.Println(k, v)
		}
	}
	fmt.Println("total", total)
}
