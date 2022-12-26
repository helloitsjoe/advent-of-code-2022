package advent07

import (
	"fmt"
	"os"
	"path"
	"reflect"
	"strings"
	"testing"
)

func TestBuildTree(t *testing.T) {
	filePath := path.Join(".", "test_input.txt")
	bytes, _ := os.ReadFile(filePath)
	content := strings.Split(strings.TrimSpace(string(bytes)), "\n")
	fmt.Println(content)
	actual := BuildTree(content)

	expected := map[string]int{
		"/":    48381165,
		"/a":   94853,
		"/a/e": 584,
		"/d":   24933642,
		// "/":     0,
		// "a":     0,
		// "e":     0,
		// "i":     584,
		// "f":     29116,
		// "g":     2557,
		// "h.lst": 62596,
		// "b.txt": 14848514,
		// "c.dat": 8504156,
		// "d":     0,
		// "j":     4060174,
		// "d.log": 8033020,
		// "d.ext": 5626152,
		// "k":     7214296,
	}

	for k, v := range actual {
		fmt.Println(k, v)
	}

	if reflect.DeepEqual(actual, expected) == false {
		t.Fatalf(`actual: %v`, actual)
	}
}
