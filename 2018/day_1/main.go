package main

import (
	readinput "adventofcode-solutions/2018/utils"
	"fmt"
	"log"
	"strconv"
)

// Part1 calculated the sum of numbers
func Part1(numbers []int) int {
	sum := 0
	for _, v := range numbers {
		sum += v
	}
	return sum
}

// Part2 finds the number the appears the second time
func Part2(numbers []int) int {
	sum := 0
	currentIdx := 0
	occurrences := make(map[int]bool)
	occurrences[sum] = true

	for {
		num := numbers[currentIdx]
		sum += num
		_, ok := occurrences[sum]
		if ok {
			return sum
		}
		occurrences[sum] = true
		// loop over
		if currentIdx == len(numbers)-1 {
			currentIdx = 0
		} else {
			currentIdx = currentIdx + 1
		}
	}
}

func main() {
	var numbers []int

	input := readinput.ReadToArray()
	for _, line := range input {
		num, err := strconv.Atoi(line)
		if err != nil {
			log.Fatal(err)
		}
		numbers = append(numbers, num)
	}

	fmt.Println(Part1(numbers))
	fmt.Println(Part2(numbers))

}
