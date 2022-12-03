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
	priority := 0
	for _, rucksack := range rucksacks {
		if strings.TrimSpace(rucksack) == "" {
			continue
		}
		middle := len(rucksack) / 2
		compartmentOne := mapset.NewSet([]byte(rucksack[0:middle])...)
		compartmentTwo := mapset.NewSet([]byte(rucksack[middle:])...)

		inBothCompartments := compartmentOne.Intersect(compartmentTwo).ToSlice()[0]

		priority += itemPriority(inBothCompartments)
	}

	return fmt.Sprint(priority), "", nil
}

func itemPriority(item byte) int {
	if item >= 'a' && item <= 'z' {
		return (int)(item - 96)
	}
	return (int)(item - 38)
}
