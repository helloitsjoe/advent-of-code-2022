package advent09

import (
	"os"
	"path"
	"strings"
	"testing"
)

func TestGetSignalFromCycle(t *testing.T) {
	filePath := path.Join("..", "advent09", "test_input.txt")
	bytes, _ := os.ReadFile(filePath)
	content := strings.Split(strings.TrimSpace(string(bytes)), "\n")

	cycles := parseCycles(content)

	expected := []int{420, 1140, 1800, 2940, 2880, 3960}

	for i, cycle := range []int{20, 60, 100, 140, 180, 220} {
		signal := getSignalByCycle(cycles, cycle)
		if signal != expected[i] {
			t.Fatalf(`Expected %v, got %v`, signal, expected[i])
		}
	}
}
