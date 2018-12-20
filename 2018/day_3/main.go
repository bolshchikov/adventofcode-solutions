package main

import (
	"adventofcode-solutions/2018/readinput"
	"fmt"
)

// Input is an interpreted input structure
type Input struct {
	id         int
	offsetLeft int
	offsetTop  int
	width      int
	height     int
}

// Coords is a struct describing coordinates of any point
type Coords struct {
	left int
	top  int
}

func inputToPoints(input Input) []Coords {
	var res []Coords
	for i := input.offsetLeft + 1; i <= input.offsetLeft+input.width; i++ {
		for j := input.offsetTop + 1; j <= input.offsetTop+input.height; j++ {
			res = append(res, Coords{i, j})
		}
	}
	return res
}

// Part1 should return the amount of overlapping points
func Part1(inputs []Input) int {
	occurrences := make(map[Coords]int)

	for _, input := range inputs {
		points := inputToPoints(input)
		for _, point := range points {
			val, ok := occurrences[point]
			if ok {
				occurrences[point] = val + 1
			} else {
				occurrences[point] = 1
			}
		}
	}

	overlapsCounter := 0
	for _, val := range occurrences {
		if val > 1 {
			overlapsCounter = overlapsCounter + 1
		}
	}
	return overlapsCounter
}

// Part2 should return id of part that is not overlapped with anything
func Part2(inputs []Input) int {
	occurrences := make(map[Coords]int)
	areas := make(map[int][]Coords)

	for _, input := range inputs {
		points := inputToPoints(input)
		areas[input.id] = points
		for _, point := range points {
			val, ok := occurrences[point]
			if ok {
				occurrences[point] = val + 1
			} else {
				occurrences[point] = 0
			}
		}
	}

	for id, points := range areas {
		var overlapSize = 0
		for _, point := range points {
			val, ok := occurrences[point]
			if ok {
				overlapSize = overlapSize + val
			}
		}
		if overlapSize == 0 {
			return id
		}
	}
	return -1
}

func main() {
	lines := readinput.ReadToArray("input.txt")
	var inputs []Input
	for _, line := range lines {
		inputs = append(inputs, ParseInput(line))
	}
	fmt.Println("Part 1: ", Part1(inputs))
	fmt.Println("Part 2: ", Part2(inputs))
}
