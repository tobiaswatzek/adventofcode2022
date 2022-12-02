package days

import (
	"adventofcode2022/util"
	"log"
	"sort"
	"strconv"
	"strings"
)

func Day1() {
	input := util.ReadFile("day1.txt")
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
	log.Printf("Second: %d\n", util.Sum(topThree))
}
