package main

import (
	"strconv"
	"strings"
	"unicode"
)

func isNotNumber(c rune) bool {
	return !unicode.IsNumber(c)
}

func extractNumber(str string) int {
	parts := strings.FieldsFunc(str, isNotNumber)
	int1, _ := strconv.Atoi(parts[0])
	return int1
}

func extractPairNumbers(str string) (int, int) {
	parts := strings.FieldsFunc(str, isNotNumber)
	int1, _ := strconv.Atoi(parts[0])
	int2, _ := strconv.Atoi(parts[1])
	return int1, int2
}

//ParseInput converts input string into a struct
func ParseInput(line string) Input {
	parts := strings.Fields(line)
	id := extractNumber(parts[0])
	offsetLeft, offsetTop := extractPairNumbers(parts[2])
	width, height := extractPairNumbers(parts[3])

	return Input{
		id,
		offsetLeft,
		offsetTop,
		width,
		height,
	}
}
