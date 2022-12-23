package advent05

import (
	"fmt"
	"log"
	"os"
	"path"
	"regexp"
	"strconv"
	"strings"
)

type Stacks = [][]string

func parseFile() (string, string) {
	filePath := path.Join("..", "advent05", "input.txt")
	file, err := os.ReadFile(filePath)
	if err != nil {
		log.Fatal("Error reading file")
	}

	contents := strings.Split(strings.TrimSpace(string(file)), "\n\n")
	return contents[0], contents[1]
}

func convertDiagramToStacks(diagram string) Stacks {
	re := regexp.MustCompile("[A-Z]")
	stacks := make(Stacks, 9)

	for _, line := range strings.Split(diagram, "\n") {
		for i, char := range line {
			// Input is e.g. [A] [B] ... so letters are at index 1, 5, 9, etc
			convertedIdx := (i - 1) / 4
			if (i-1)%4 == 0 && re.MatchString(string(char)) {
				// Append, stack is reversed to simplify pop
				stacks[convertedIdx] = append(stacks[convertedIdx], string(char))
			}
		}
	}

	// fmt.Println(stacks)
	return stacks
}

// func runSingleInstruction(line string, stacks Stacks) {
// 	// move n from i to j
// 	split := strings.Split(line, " ")
// 	num, _ := strconv.Atoi(split[1])
// 	from, _ := strconv.Atoi(split[3])
// 	to, _ := strconv.Atoi(split[5])

// 	from, to = from-1, to-1

// 	for n := 0; n < num; n++ {
// 		// Prepend, top of stack is index 0
// 		stacks[to] = append([]string{stacks[from][0]}, stacks[to]...)
// 		stacks[from] = stacks[from][1:]
// 	}
// }

func runSingleInstructionPartTwo(line string, stacks Stacks) {
	// move n from i to j
	split := strings.Split(line, " ")
	num, _ := strconv.Atoi(split[1])
	from, _ := strconv.Atoi(split[3])
	to, _ := strconv.Atoi(split[5])

	from, to = from-1, to-1

	// Prepend, top of stack is index 0
	temp := make([]string, len(stacks[from])-num)
	copy(temp, stacks[from][num:])

	toAdd := stacks[from][0:num]
	fmt.Println("To add", toAdd)
	fmt.Println("New from", temp)

	stacks[to] = append(toAdd, stacks[to]...)
	stacks[from] = temp
}

func runInstructions(input string, stacks Stacks) {
	for _, line := range strings.Split(input, "\n") {
		// if i > 3 {
		// 	fmt.Println("")
		// 	break
		// }
		// fmt.Println(line)
		// for _, stack := range stacks {
		// 	fmt.Println(stack)
		// }
		runSingleInstructionPartTwo(line, stacks)
	}
}

func Run() {
	diagram, instructions := parseFile()
	stacks := convertDiagramToStacks(diagram)
	runInstructions(instructions, stacks)

	for _, stack := range stacks {
		fmt.Println(stack)
	}

	topCrates := ""
	for _, stack := range stacks {
		topCrates += stack[0]
	}
	fmt.Println(topCrates)
}
