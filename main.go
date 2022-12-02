package main

import (
	"errors"
	"io/ioutil"
	"log"
	"sort"
	"strconv"
	"strings"
)

func main() {
	log.Println("AoC 2022")
	day1()
	day2()
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

func day2() {
	input := readFile("day2.txt")
	lines := strings.Split(input, "\n")

	score := 0

	for _, line := range lines {
		if strings.TrimSpace(line) == "" {
			continue
		}

		columns := strings.Split(line, " ")
		opponentMove, err := parseColumnOne(columns[0])
		if err != nil {
			log.Fatal(err)
		}
		myMove, err := parseColumnTwo(columns[1])
		if err != nil {
			log.Fatal(err)
		}
		score += shapeScore(myMove)
		score += roundScore(opponentMove, myMove)
	}

	log.Println("Day 2")
	log.Printf("First: %d\n", score)
}

type shape int

const (
	rock shape = iota
	paper
	scissors
)

func roundScore(opponentMove, myMove shape) int {
	if opponentMove == myMove {
		return 3
	}

	if opponentMove == rock && myMove == scissors {
		return 0
	}

	if opponentMove == paper && myMove == rock {
		return 0
	}

	if opponentMove == scissors && myMove == paper {
		return 0
	}

	return 6
}

func shapeScore(s shape) int {
	switch s {
	case rock:
		return 1
	case paper:
		return 2
	case scissors:
		return 3
	default:
		panic("Shape is not known")
	}
}

func parseColumnOne(c string) (shape, error) {
	switch c {
	case "A":
		return rock, nil
	case "B":
		return paper, nil
	case "C":
		return scissors, nil
	}

	return rock, errors.New("Cannot parse move")
}

func parseColumnTwo(c string) (shape, error) {
	switch c {
	case "X":
		return rock, nil
	case "Y":
		return paper, nil
	case "Z":
		return scissors, nil
	}

	return rock, errors.New("Cannot parse move")
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
