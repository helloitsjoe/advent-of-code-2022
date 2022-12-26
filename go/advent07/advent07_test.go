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
	}

	for k, v := range actual {
		fmt.Println(k, v)
	}

	if reflect.DeepEqual(actual, expected) == false {
		t.Fatalf(`actual: %v`, actual)
	}
}

func TestFindBestFolderToDelete(t *testing.T) {
	tree := map[string]int{
		"/":    48381165,
		"/a":   94853,
		"/a/e": 584,
		"/d":   24933642,
	}
	_, size := FindDirToDelete(tree)
	if size != 24933642 {
		t.Fatalf(`dirName: %v`, size)
	}
}
