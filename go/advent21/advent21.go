package advent21

import (
	"fmt"
	"log"
	"os"
	"path"
	"strconv"
	"strings"
)

type MonkeyMap = map[string]string

func convert(value string, obj MonkeyMap) int {
	numOrExpression, err := strconv.Atoi(value)
	if err == nil {
		// We have a number
		return numOrExpression
	}

	// Need to recursively evaluate expression
	parsed := strings.Split(value, " ")
	head, oper, tail := parsed[0], parsed[1], parsed[2]

	rHead := convert(obj[head], obj)
	rTail := convert(obj[tail], obj)

	foo := map[string]int{
		"+": rHead + rTail,
		"-": rHead - rTail,
		"*": rHead * rTail,
		"/": rHead / rTail,
	}

	return foo[oper]
}

func Run() {
	path := path.Join("..", "advent21", "input.txt")
	file, err := os.ReadFile(path)

	if err != nil {
		log.Fatalf("Error reading file: %v", err)
	}

	trimmed := strings.TrimSpace(string(file))
	lines := strings.Split(trimmed, "\n")

	kvMap := make(MonkeyMap)

	for _, line := range lines {
		keyValue := strings.Split(line, ": ")
		kvMap[keyValue[0]] = keyValue[1]
	}

	total := convert(kvMap["root"], kvMap)
	fmt.Println(total)
}
