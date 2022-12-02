package days

import (
	"adventofcode2022/util"
	"errors"
	"log"
	"strings"
)

func Day2() {
	input := util.ReadFile("day2.txt")
	lines := strings.Split(input, "\n")

	scorePartOne := 0
	scorePartTwo := 0

	for _, line := range lines {
		if strings.TrimSpace(line) == "" {
			continue
		}

		columns := strings.Split(line, " ")

		// Part 1
		opponentMove, err := parseColumnOne(columns[0])
		if err != nil {
			log.Fatal(err)
		}
		myMove, err := parseColumnTwoToMove(columns[1])
		if err != nil {
			log.Fatal(err)
		}
		scorePartOne += shapeScore(myMove)
		scorePartOne += roundScore(opponentMove, myMove)

		// Part Two
		expectedOutcome, err := parseColumnTwoToOutcome(columns[1])
		if err != nil {
			log.Fatal(err)
		}
		myShape := shapeForOutcome(expectedOutcome, opponentMove)
		scorePartTwo += shapeScore(myShape)
		scorePartTwo += roundScore(opponentMove, myShape)
	}

	log.Println("Day 2")
	log.Printf("First: %d\n", scorePartOne)
	log.Printf("Second: %d\n", scorePartTwo)
}

type outcome int

const (
	lose outcome = iota
	draw
	win
)

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

func shapeForOutcome(expectedOutcome outcome, opponentMove shape) shape {
	if expectedOutcome == draw {
		return opponentMove
	}

	if expectedOutcome == win {
		switch opponentMove {
		case rock:
			return paper
		case paper:
			return scissors
		case scissors:
			return rock
		default:
			panic("Unknown move")
		}
	}

	switch opponentMove {
	case rock:
		return scissors
	case paper:
		return rock
	case scissors:
		return paper
	default:
		panic("Unknown move")
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

func parseColumnTwoToMove(c string) (shape, error) {
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

func parseColumnTwoToOutcome(c string) (outcome, error) {
	switch c {
	case "X":
		return lose, nil
	case "Y":
		return draw, nil
	case "Z":
		return win, nil
	}

	return lose, errors.New("Cannot parse outcome")
}
