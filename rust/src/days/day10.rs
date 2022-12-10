use std::{fs, path::PathBuf};

pub fn solve(input_path: &PathBuf) -> (String, String) {
    let input = fs::read_to_string(input_path).unwrap();

    let instructions = parse_instructions(&input);
    let part_one = solve_part_one(&instructions);

    (part_one.to_string(), String::new())
}

fn solve_part_one(instructions: &Vec<Instruction>) -> i32 {
    let mut x = 1;
    let mut cycle = 1;
    let mut signal_strength = 0;
    let mut inspection_interval = 20;

    for instruction in instructions {
        let wait_time = match instruction {
            Instruction::AddX(_) => 2,
            Instruction::Noop => 1,
        };

        for _ in 0..wait_time {
           // println!("{cycle:3}: {x:3} ({signal_strength:4}) | {instruction:?}");

            // TODO: first 20 then after 40
            if cycle == inspection_interval {
                signal_strength += cycle * x;
                inspection_interval += 40;
            }

            cycle += 1;
        }

        x = match instruction {
            Instruction::AddX(a) => x + a,
            Instruction::Noop => x,
        };
    }

    return signal_strength;
}

#[derive(Debug)]
enum Instruction {
    AddX(i32),
    Noop,
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            let parts = l.split(" ").collect::<Vec<_>>();
            match parts[0] {
                "addx" => Instruction::AddX(parts[1].parse::<i32>().unwrap()),
                "noop" => Instruction::Noop,
                _ => panic!("unknown instruction {l}"),
            }
        })
        .collect()
}
