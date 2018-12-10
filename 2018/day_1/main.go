package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	var numbers []int
	for scanner.Scan() {
		line := scanner.Text()
		num, err := strconv.Atoi(line)
		if err != nil {
			log.Fatal(err)
		}
		numbers = append(numbers, num)
	}

	sum := 0
	for _, v := range numbers {
		sum += v
	}
	fmt.Println(sum)

	sum = 0
	idx := 0
	occurrences := make(map[int]bool)
	occurrences[sum] = true
	for {
		num := numbers[idx]
		sum += num
		_, ok := occurrences[sum]
		if ok {
			fmt.Println(sum)
			break
		}
		occurrences[sum] = true
		if idx == len(numbers)-1 {
			idx = 0
		} else {
			idx = idx + 1
		}
	}
}
