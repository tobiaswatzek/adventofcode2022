package main

import (
	"adventofcode2022/days"
	"fmt"
	"os"
	"strconv"
)

func main() {
	fmt.Println("AoC 2022")
	dayNumber := parseDayNumberFromArgs()
	inputFilePath := inputFilePathForDay(dayNumber)
	solutionPartOne, solutionPartTwo, err := days.RunDay(days.RunDayData{Number: dayNumber, InputFilePath: inputFilePath})
	if err != nil {
		panic(err)
	}
	fmt.Printf("Day %d\n\tPart One: %s\n\tPart Two:%s\n", dayNumber, solutionPartOne, solutionPartTwo)
}

func inputFilePathForDay(dayNumber int) string {
	return fmt.Sprintf("day%d.txt", dayNumber)
}

func parseDayNumberFromArgs() int {
	if len(os.Args[1:]) != 1 {
		panic("Expecting exactly one argument.")
	}

	dayNumber, err := strconv.Atoi(os.Args[1])
	if err != nil {
		panic(err)
	}

	if dayNumber < 1 || dayNumber > 24 {
		panic("Only accepting numbers between 1 and 24")
	}

	return dayNumber
}
