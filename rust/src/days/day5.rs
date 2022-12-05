use std::{
    collections::{HashMap, VecDeque},
    fs,
    path::PathBuf,
};

pub fn solve(input_path: &PathBuf) -> (String, String) {
    let input = fs::read_to_string(input_path).expect("should have been able to read the file");

    // let stacks = parse_keys(input);
    let instructions: Vec<CraneInstruction> = parse_instructions(&input);

    let part_one = solve_part_one(&mut build_stacks(), &instructions);
    let part_two = solve_part_two(&mut build_stacks(), &instructions);

    return (part_one, part_two);
}

fn solve_part_one(
    stacks: &mut Vec<VecDeque<char>>,
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
    stacks: &mut Vec<VecDeque<char>>,
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

fn build_stacks() -> Vec<VecDeque<char>> {
    let mut stacks = Vec::new();

    stacks.push(VecDeque::from(['G', 'B', 'D', 'C', 'P', 'R']));
    stacks.push(VecDeque::from(['G', 'V', 'H']));
    stacks.push(VecDeque::from(['M', 'P', 'J', 'D', 'Q', 'S', 'N']));
    stacks.push(VecDeque::from(['M', 'N', 'C', 'D', 'G', 'L', 'S', 'P']));
    stacks.push(VecDeque::from(['S', 'L', 'F', 'P', 'C', 'N', 'B', 'J']));
    stacks.push(VecDeque::from(['S', 'T', 'G', 'V', 'Z', 'D', 'B', 'Q']));
    stacks.push(VecDeque::from(['Q', 'T', 'F', 'H', 'M', 'Z', 'B']));
    stacks.push(VecDeque::from(['F', 'B', 'D', 'M', 'C']));
    stacks.push(VecDeque::from(['G', 'Q', 'C', 'F']));

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

fn parse_keys(input: String) -> HashMap<u8, VecDeque<char>> {
    input
        .lines()
        .find(|l| l.starts_with(" 1"))
        .and_then(|l| {
            let keys = l
                .trim()
                .split("   ")
                .map(|s| s.parse::<u8>().expect("key must be a number"));

            let mut map: HashMap<u8, VecDeque<char>> = HashMap::new();

            for key in keys {
                map.insert(key, VecDeque::new());
            }

            if map.is_empty() {
                return None;
            }

            Some(map)
        })
        .expect("keys must be found")
}
