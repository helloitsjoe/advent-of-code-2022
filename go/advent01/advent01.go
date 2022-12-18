package advent01

import (
	"fmt"
	"log"
	"os"
	"path/filepath"
	"strconv"
	"strings"
)

// type maxHeap struct {
// 	maxValue int
// }

// func (m *maxHeap) insert(i int) {
// 	if n.value > i {
// 		// push it down
// 	} else {
// 		// push head down
// 	}
// }

// func (m *maxHeap) getMax() int {
// 	return m.maxValue
// }

// func (m *maxHeap) removeMax() int {

// }

// func (m *maxHeap) heapify() int {

// }

func Run() {
	path := filepath.Join("..", "advent01", "input.txt")
	body, err := os.ReadFile(path)

	if err != nil {
		log.Fatalf("Unable to read file: %v", err)
	}

	text := strings.TrimSpace(string(body))
	t := strings.Split(text, "\n\n")

	c := make([]int, len(t))
	max := make([]int, 3)
	for i, line := range t {
		total := 0
		for _, num := range strings.Split(line, "\n") {
			int, err := strconv.Atoi(num)
			if err != nil {
				log.Fatalf("Could not convert to int: %v", err)
			}
			total += int
		}
		if total > max[0] {
			max[2] = max[1]
			max[1] = max[0]
			max[0] = total
		} else if total > max[1] {
			max[2] = max[1]
			max[1] = total
		} else if total > max[2] {
			max[2] = total
		}
		c[i] = total
	}

	grandTotal := 0
	for _, ea := range max {
		grandTotal += ea
	}
	fmt.Println(grandTotal)
}
