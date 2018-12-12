package readinput

import (
	"bufio"
	"log"
	"os"
)

// ReadToArray parses input file into array by line
func ReadToArray() []string {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	var res []string
	for scanner.Scan() {
		line := scanner.Text()
		res = append(res, line)
	}

	return res
}
