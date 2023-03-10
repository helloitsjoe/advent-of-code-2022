package advent09

import (
	"fmt"
	"os"
	"path"
	"strconv"
	"strings"
)

func getXByCycle(cycles []int, cycleNum int) int {
	x := 1
	for i, cycle := range cycles {
		if i == cycleNum-1 {
			return x
		}
		x += cycle
	}
	return x
}

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

func getSumOfCycles(cycles []int) int {
	sum := 0
	for _, cycle := range []int{20, 60, 100, 140, 180, 220} {
		signal := getSignalByCycle(cycles, cycle)
		fmt.Println(signal)
		sum += signal
	}
	return sum
}

func createImageOnCRT(cycles []int) []string {
	image := make([]string, 6)
	fmt.Println(cycles)

	for i := range image {
		curr := ""
		for j := 1; j < 41; j++ {
			x := getXByCycle(cycles, j+(40*i))
			h, m, t := x-1, x, x+1
			fmt.Println(h, m, t, j, x)
			if j-1 == h || j-1 == m || j-1 == t {
				curr += "#"
			} else {
				curr += "."
			}
		}
		image[i] = curr
	}

	return image
}

func Run() {
	filePath := path.Join("..", "advent09", "input.txt")
	bytes, _ := os.ReadFile(filePath)
	content := strings.Split(strings.TrimSpace(string(bytes)), "\n")

	cycles := parseCycles(content)

	// Part 1
	// sum := getSumOfCycles(cycles)
	// fmt.Println(sum)

	// Part 2
	image := createImageOnCRT(cycles)
	for _, line := range image {
		fmt.Println(line)
	}
}
