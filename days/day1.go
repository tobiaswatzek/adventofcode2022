package days

import (
	"adventofcode2022/util"
	"fmt"
	"sort"
	"strconv"
	"strings"
)

func day1(inputFilePath string) (string, string, error) {
	input := util.ReadFile(inputFilePath)
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
				return "", "", err
			}
			current += number
		}
	}

	sort.Slice(caloriesPerElf, func(i, j int) bool { return caloriesPerElf[i] > caloriesPerElf[j] })
	topThree := caloriesPerElf[0:3]

	firstPart := fmt.Sprint(topThree[0])
	secondPart := fmt.Sprint(util.Sum(topThree))

	return firstPart, secondPart, nil
}
