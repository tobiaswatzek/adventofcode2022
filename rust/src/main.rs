use std::fs;

fn main() {
    let file_path = "../data/day1.txt";

    let (part_one, part_two) = day1(file_path);

    println!("Day 1:\n\tPart one: {part_one}\n\tPart two: {part_two}");
}


fn day1(input_path: &str) -> (String, String)  {
    let input = fs::read_to_string(input_path).expect("Should have been able to read the file");

    let mut calories_per_elf = input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|l| l.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();
    calories_per_elf.sort_unstable();

    let max_calories = calories_per_elf.last().expect("There has to be an element");
    let sum_max_three: i32 = calories_per_elf.iter().rev().take(3).sum();

    (max_calories.to_string(), sum_max_three.to_string())
}
