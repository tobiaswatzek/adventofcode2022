package main

import (
	"io/ioutil"
	"log"
	"strconv"
	"strings"
)

func main() {
	log.Println("AoC 2022")
	day1()
}

func day1() {
	input := readFile("day1.txt")
	lines := strings.Split(input, "\n")

	current := 0
	most := 0

	for _, line := range lines {
		if line == "" {
			if current > most {
				most = current
			}
			current = 0
		} else {
			number, err := strconv.Atoi(line)
			if err != nil {
				log.Fatal(err)
			}
			current += number
		}
	}

	log.Println("Day 1")
	log.Printf("First: %d\n", most)
}

func readFile(filepath string) string {
	data, err := ioutil.ReadFile(filepath)
	if err != nil {
		log.Fatal(err)
	}

	return string(data)
}
