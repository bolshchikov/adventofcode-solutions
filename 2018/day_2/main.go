package main

import (
	"adventofcode-solutions/2018/readinput"
	"fmt"
)

// Part1 should calculate the multiplication of amount of string that has 2 repetitions and 3 repetitions
func Part1(input []string) int {
	var numberOfTwices = 0
	var numberOfTripples = 0

	for _, line := range input {
		occurrences := make(map[rune]int)
		for _, char := range line {
			val, ok := occurrences[char]
			if ok {
				occurrences[char] = val + 1
			} else {
				occurrences[char] = 1
			}
		}

		var foundTwo = false
		var foundThree = false
		for _, value := range occurrences {
			if value == 2 && !foundTwo {
				numberOfTwices = numberOfTwices + 1
				foundTwo = true
			}
			if value == 3 && !foundThree {
				numberOfTripples = numberOfTripples + 1
				foundThree = true
			}
		}
	}
	return numberOfTwices * numberOfTripples
}

func findSimilarLines(input []string) (string, string) {
	for i := 0; i < len(input); i++ {
		for j := i + 1; j < len(input); j++ {
			lineA := input[i]
			lineB := input[j]
			numberOfErrors := 0
			for k := 0; k < len(lineA); k++ {
				if lineA[k] != lineB[k] {
					numberOfErrors++
				}
			}
			if numberOfErrors == 1 {
				return lineA, lineB
			}
		}
	}
	return "", ""
}

// Part2 returns the overlap of chars between two strings
func Part2(input []string) string {
	lineA, lineB := findSimilarLines(input)
	var res = []byte{}
	for i := 0; i < len(lineA); i++ {
		if lineA[i] == lineB[i] {
			res = append(res, lineA[i])
		}
	}
	return string(res[:])
}

func main() {
	input := readinput.ReadToArray("input.txt")
	res1 := Part1(input)
	res2 := Part2(input)
	fmt.Println("Part 1:", res1)
	fmt.Println("Part 2:", res2)
}
