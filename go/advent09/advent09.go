package advent09

import (
	"fmt"
	"os"
	"path"
	"strconv"
	"strings"
)

func getSignalByCycle(cycles []int, cycleNum int) int {
	x := 1
	for i, cycle := range cycles {
		if i == cycleNum-1 {
			return x * cycleNum
		}
		x += cycle
	}
	return x
}

func parseCycles(content []string) []int {
	cycles := []int{}
	for _, line := range content {
		ea := strings.Split(line, " ")
		if len(ea) == 2 {
			_, valStr := ea[0], ea[1]
			val, _ := strconv.Atoi(valStr)
			cycles = append(cycles, []int{0, val}...)
		} else {
			cycles = append(cycles, 0)
		}
	}
	return cycles
}

func Run() {
	filePath := path.Join("..", "advent09", "input.txt")
	bytes, _ := os.ReadFile(filePath)
	content := strings.Split(strings.TrimSpace(string(bytes)), "\n")

	cycles := parseCycles(content)

	sum := 0
	for _, cycle := range []int{20, 60, 100, 140, 180, 220} {
		signal := getSignalByCycle(cycles, cycle)
		fmt.Println(signal)
		sum += signal
	}
	fmt.Println(sum)
}
