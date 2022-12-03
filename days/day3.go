package days

import (
	"adventofcode2022/util"
	"fmt"
	"strings"

	mapset "github.com/deckarep/golang-set/v2"
)

func day3(inputFilePath string) (string, string, error) {
	input := util.ReadFile(inputFilePath)
	rucksacks := strings.Split(input, "\n")

	solutionOne := solveDay3PartOne(rucksacks[:len(rucksacks)-1])
	solutionTwo := solveDay3PartTwo(rucksacks[:len(rucksacks)-1])

	return fmt.Sprint(solutionOne), fmt.Sprint(solutionTwo), nil
}

func solveDay3PartOne(rucksacks []string) int {
	priority := 0
	for _, rucksack := range rucksacks {
		middle := len(rucksack) / 2
		compartmentOne := mapset.NewSet([]byte(rucksack[0:middle])...)
		compartmentTwo := mapset.NewSet([]byte(rucksack[middle:])...)

		inBothCompartments := compartmentOne.Intersect(compartmentTwo).ToSlice()[0]

		priority += itemPriority(inBothCompartments)
	}

	return priority
}

func solveDay3PartTwo(rucksacks []string) int {
	badge := mapset.NewSet[byte]()
	priority := 0
	for i, rucksack := range rucksacks {
		items := mapset.NewSet([]byte(rucksack)...)

		if i%3 == 0 {
			badge = items
		} else {
			badge = badge.Intersect(items)
		}

		if i%3 == 2 {
			priority += itemPriority(badge.ToSlice()[0])
		}
	}

	return priority
}

func itemPriority(item byte) int {
	if item >= 'a' && item <= 'z' {
		return (int)(item - 96)
	}
	return (int)(item - 38)
}
