package advent08

import (
	"fmt"
	"log"
	"os"
	"path"
	"strconv"
	"strings"
)

func getVal(b byte) (int, error) {
	return strconv.Atoi(string(b))
}

func FindInvisibleTrees(content []string) []string {
	invisibleFromLeft := map[string]bool{}
	invisibleFromRight := map[string]bool{}
	invisibleFromTop := map[string]bool{}
	invisibleFromBottom := map[string]bool{}

	invisible := []string{}

	for y := 0; y < len(content); y++ {
		highestLeft := -1
		highestRight := -1

		for x := 0; x < len(content[y]); x++ {
			currLeft, _ := getVal(content[y][x])
			currRight, _ := getVal(content[y][len(content[y])-1-x])

			if currLeft > highestLeft {
				highestLeft = currLeft
			} else if x != 0 {
				invisibleFromLeft[fmt.Sprint(x)+":"+fmt.Sprint(y)] = true
			}

			if currRight > highestRight {
				highestRight = currRight
			} else if x != len(content[y])-1 {
				invisibleFromRight[fmt.Sprint(len(content[y])-1-x)+":"+fmt.Sprint(y)] = true
			}
		}
	}

	for x := 0; x < len(content[0]); x++ {
		highestTop := -1
		highestBottom := -1

		for y := 0; y < len(content); y++ {
			currTop, _ := getVal(content[y][x])
			currBottom, _ := getVal(content[len(content)-1-y][x])

			if currTop > highestTop {
				highestTop = currTop
			} else if y != 0 {
				invisibleFromTop[fmt.Sprint(x)+":"+fmt.Sprint(y)] = true
			}

			if currBottom > highestBottom {
				highestBottom = currBottom
			} else if y != len(content)-1 {
				invisibleFromBottom[fmt.Sprint(x)+":"+fmt.Sprint(len(content)-1-y)] = true
			}
		}
	}

	for k, _ := range invisibleFromLeft {
		if invisibleFromRight[k] == true && invisibleFromTop[k] == true && invisibleFromBottom[k] == true {
			invisible = append(invisible, k)
		}
	}

	return invisible
}

func getTotalVisible(content []string, invisibleCoords []string) int {
	return len(content)*len(content[0]) - len(invisibleCoords)
}

// === Part 2 ===

func calculateScenicScore(content []string) int {
	highScore := 0
	for y := 1; y < len(content)-1; y++ {
		for x := 1; x < len(content[y])-1; x++ {
			curr, _ := getVal(content[y][x])

			leftScore := 1
			rightScore := 1
			topScore := 1
			bottomScore := 1

			for i := 1; i < len(content[y]); i++ {
				if x+i >= len(content[y]) {
					continue
				}
				currLeft, _ := getVal(content[y][x+i])

				if currLeft >= curr {
					break
				}
				fmt.Println("currLeft:", currLeft)
				leftScore += 1
			}

			for k := 1; k < len(content[y]); k++ {
				if x-k < 0 {
					continue
				}
				currRight, _ := getVal(content[y][x-k])
				if currRight >= curr {
					break
				}
				rightScore += 1
			}

			for j := 1; j < len(content); j++ {
				if y+j >= len(content) {
					continue
				}
				currTop, _ := getVal(content[y+j][x])
				if currTop >= curr {
					break
				}
				topScore += 1
			}

			for l := 1; l < len(content); l++ {
				if y-l < 0 {
					continue
				}
				currBottom, _ := getVal(content[y-l][x])
				if currBottom >= curr {
					break
				}
				bottomScore += 1
			}

			score := leftScore * rightScore * topScore * bottomScore
			fmt.Println("curr:", curr)
			fmt.Println("score:", score)
			fmt.Println(leftScore, rightScore, topScore, bottomScore)
			if score > highScore {
				highScore = score
			}
		}
	}
	return highScore
}

func Run() {
	filePath := path.Join("..", "advent08", "input.txt")
	bytes, err := os.ReadFile(filePath)
	if err != nil {
		log.Fatal("Error reading file")
	}
	content := strings.Split(strings.TrimSpace(string(bytes)), "\n")

	// Part 1
	// invisibleCoords := FindInvisibleTrees(content)
	// totalVisible := getTotalVisible(content, invisibleCoords)
	// fmt.Println("visible:", totalVisible)

	// Part 2
	scenicScore := calculateScenicScore(content)
	fmt.Println("Scenic score:", scenicScore)
}
