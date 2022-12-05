use std::{
    collections::{VecDeque},
    fs,
    path::PathBuf,
};

pub fn solve(input_path: &PathBuf) -> (String, String) {
    let input = fs::read_to_string(input_path).expect("should have been able to read the file");
    let instructions: Vec<CraneInstruction> = parse_instructions(&input);

    let part_one = solve_part_one(&mut build_stacks(&input), &instructions);
    let part_two = solve_part_two(&mut build_stacks(&input), &instructions);

    return (part_one, part_two);
}

fn solve_part_one(
    stacks: &mut [VecDeque<char>; 9],
    instructions: &Vec<CraneInstruction>,
) -> String {
    for instruction in instructions {
        for _ in 0..instruction.count {
            let item = stacks[instruction.from - 1]
                .pop_front()
                .expect("element is expected");
            stacks[instruction.to - 1].push_front(item);
        }
    }

    stacks.iter().map(|s| s.front()).filter_map(|s| s).collect()
}

fn solve_part_two(
    stacks: &mut [VecDeque<char>; 9],
    instructions: &Vec<CraneInstruction>,
) -> String {
    for instruction in instructions {
        let mut items: Vec<char> = Vec::new();
        for _ in 0..instruction.count {
            let item = stacks[instruction.from - 1]
                .pop_front()
                .expect("element is expected");
            items.push(item);
        }
        items
            .iter()
            .rev()
            .for_each(|item| stacks[instruction.to - 1].push_front(*item));
    }

    stacks.iter().map(|s| s.front()).filter_map(|s| s).collect()
}

fn build_stacks(input: &str) -> [VecDeque<char>; 9] {
    let mut stacks: [VecDeque<char>; 9] = Default::default();

    for line in input.lines().take(8) {
        line.chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .map(|chunk| chunk.iter().filter(|c| c.is_alphabetic()).nth(0))
            .enumerate()
            .for_each(|(i, item)| match item {
                Some(c) => stacks[i].push_back(c.to_owned()),
                None => (),
            });
    }

    stacks
}

fn parse_instructions(input: &str) -> Vec<CraneInstruction> {
    input
        .lines()
        .skip(10)
        .map(|l| parse_instruction_line(l))
        .collect()
}

fn parse_instruction_line(line: &str) -> CraneInstruction {
    let words = line.split(" ").collect::<Vec<_>>();
    let count = words[1].parse().expect("must be able to parse count");
    let from = words[3].parse().expect("must be able to parse from");
    let to = words[5].parse().expect("must be able to parse to");

    CraneInstruction { count, from, to }
}

#[derive(Debug)]
struct CraneInstruction {
    count: usize,
    from: usize,
    to: usize,
}
