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

const TOTAL_SPACE = 70000000
const REQUIRED_SPACE = 30000000

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
				if target == ".." && currDir != "/" {
					dirs[currDir] += childTotal
				}
			}
		} else {
			temp := strings.Split(line, " ")
			idOrSize := temp[0]
			if idOrSize != "dir" {
				size, _ := strconv.Atoi(idOrSize)
				if currDir != "/" {
					dirs[currDir] += size
				}
				dirs["/"] += size
			}
		}
		// fmt.Println("dirs", dirs, line)
	}

	return dirs
}

func FindDirToDelete(tree Dir) (string, int) {
	target := tree["/"] - REQUIRED_SPACE
	bestSize := TOTAL_SPACE
	dirName := ""

	for k, v := range tree {
		if v >= target && v < bestSize {
			fmt.Println(k, v)
			bestSize = v
			dirName = k
		}
	}

	fmt.Println("target", target)
	fmt.Println("/:", tree["/"])
	return dirName, bestSize
}

func Run() {
	inputPath := path.Join("..", "advent07", "input.txt")
	file, err := os.ReadFile(inputPath)
	if err != nil {
		log.Fatal("Error reading file")
	}
	content := string(file)

	// TODO: HMM... not sure why this isn't producing the right answer TBH
	// Test passes with the example value
	tree := BuildTree(strings.Split(strings.TrimSpace(content), "\n"))

	dirName, bestSize := FindDirToDelete(tree)

	fmt.Println("dirName", dirName)
	fmt.Println("bestSize", bestSize)
}
