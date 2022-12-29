package advent08

import (
	"os"
	"path"
	// "reflect"
	"strings"
	"testing"
)

// func TestFindInvisibleTrees(t *testing.T) {
// 	filePath := path.Join("test_input.txt")
// 	bytes, _ := os.ReadFile(filePath)
// 	content := strings.Split(strings.TrimSpace(string(bytes)), "\n")
// 	invisible := FindInvisibleTrees(content)
// 	expected := []string{}

// 	if reflect.DeepEqual(invisible, expected) == false {
// 		t.Fatal("invisible", invisible)
// 	}
// }

func TestGetTotalVisible(t *testing.T) {
	filePath := path.Join("test_input.txt")
	bytes, _ := os.ReadFile(filePath)
	content := strings.Split(strings.TrimSpace(string(bytes)), "\n")
	invisible := FindInvisibleTrees(content)

	totalVisible := getTotalVisible(content, invisible)

	if totalVisible != 21 {
		t.Fatal("totalVisible", totalVisible)
	}
}

func TestScenicScore(t *testing.T) {
	filePath := path.Join("test_input.txt")
	bytes, _ := os.ReadFile(filePath)
	content := strings.Split(strings.TrimSpace(string(bytes)), "\n")
	score := calculateScenicScore(content)

	if score != 8 {
		t.Fatal("score", score)
	}
}
