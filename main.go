package main

import (
	"io/ioutil"
	"log"
	"sort"
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

	caloriesPerElf := []int{}

	current := 0
	for _, line := range lines {
		if line == "" {
			caloriesPerElf = append(caloriesPerElf, current)
			current = 0
		} else {
			number, err := strconv.Atoi(line)
			if err != nil {
				log.Fatal(err)
			}
			current += number
		}
	}

	sort.Slice(caloriesPerElf, func(i, j int) bool { return caloriesPerElf[i] > caloriesPerElf[j] })
	topThree := caloriesPerElf[0:3]
	log.Println("Day 1")
	log.Printf("First: %d\n", topThree[0])
	log.Printf("Second: %d\n", sum(topThree))
}

func readFile(filepath string) string {
	data, err := ioutil.ReadFile(filepath)
	if err != nil {
		log.Fatal(err)
	}

	return string(data)
}

func sum(numbers []int) int {
	var sum int
	for _, number := range numbers {
		sum += number
	}
	return sum
}
