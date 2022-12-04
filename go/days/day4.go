package days

import (
	"adventofcode2022/util"
	"fmt"
	"strconv"
	"strings"
)

func day4(inputFilePath string) (string, string, error) {
	input := util.ReadFile(inputFilePath)
	lines := strings.Split(input, "\n")

	fullyContainsCount := 0
	overlapsCount := 0
	for _, line := range lines {
		if strings.TrimSpace(line) == "" {
			continue
		}
		parts := strings.Split(line, ",")

		firstRange, err := day4ParsePart(parts[0])
		if err != nil {
			return "", "", err
		}
		secondRange, err := day4ParsePart(parts[1])
		if err != nil {
			return "", "", err
		}

		if firstRange.fullyContains(secondRange) || secondRange.fullyContains(firstRange) {
			fullyContainsCount++
		}

		if firstRange.overlaps(secondRange) {
			overlapsCount++
		}

	}

	return fmt.Sprint(fullyContainsCount), fmt.Sprint(overlapsCount), nil
}

type sectionRange struct {
	lower int
	upper int
}

func (a sectionRange) fullyContains(other sectionRange) bool {
	return a.lower <= other.lower && a.upper >= other.upper
}

func (a sectionRange) overlaps(b sectionRange) bool {
	return a.lower == b.lower ||
		a.lower == b.upper ||
		a.upper == b.lower ||
		a.upper == b.upper ||
		(a.lower < b.lower && b.lower < a.upper) ||
		(b.lower < a.lower && a.lower < b.upper)
}

func day4ParsePart(part string) (sectionRange, error) {
	ids := strings.Split(part, "-")

	lower, err := strconv.Atoi(ids[0])
	if err != nil {
		return sectionRange{}, err
	}
	upper, err := strconv.Atoi(ids[1])
	if err != nil {
		return sectionRange{}, err
	}

	return sectionRange{lower: lower, upper: upper}, nil
}
