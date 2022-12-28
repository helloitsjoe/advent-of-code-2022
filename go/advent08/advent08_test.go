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

	totalVisible := getTotalVisible(content, len(invisible))

	if totalVisible != 5 {
		t.Fatal("totalVisible", totalVisible)
	}
}
