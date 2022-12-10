use std::{collections::HashSet, fs, iter, ops::Neg, path::PathBuf, str::Lines};

pub fn solve(input_path: &PathBuf) -> (String, String) {
    let input = fs::read_to_string(input_path).expect("file must be readable");
    let instructions = parse_instructions(input.lines());

    let part_one = solve_part_one(&instructions);

    (part_one.to_string(), String::new())
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Position {
    x: i32,
    y: i32,
}

fn solve_part_one(instructions: &Vec<Instruction>) -> usize {
    let mut visited_positions: HashSet<Position> = HashSet::new();
    let mut head = Position { x: 0, y: 0 };
    let mut tail = Position { x: 0, y: 0 };

    visited_positions.insert(tail);
    for (i, instruction) in instructions.iter().enumerate() {

        head = match instruction {
            Instruction::Up => move_up(head),
            Instruction::Down => move_down(head),
            Instruction::Left => move_left(head),
            Instruction::Right => move_right(head),
        };

        if are_touching(&head, &tail) {
            continue;
        }

        let offset = match instruction {
            Instruction::Up => 1,
            Instruction::Down => -1,
            Instruction::Left => -1,
            Instruction::Right => 1,
        };

        if head.x == tail.x {
            tail = Position {
                y: tail.y + offset,
                ..tail
            };
            visited_positions.insert(tail);
            continue;
        }

        if head.y == tail.y {
            tail = Position {
                x: tail.x + offset,
                ..tail
            };
            visited_positions.insert(tail);
            continue;
        }

        tail = match instruction {
            Instruction::Up => Position { x: head.x, y: head.y - 1 },
            Instruction::Down => Position { x: head.x, y: head.y + 1 },
            Instruction::Left => Position { x: head.x + 1, y: head.y },
            Instruction::Right => Position { x: head.x - 1, y: head.y },
        };

        visited_positions.insert(tail);
    }

    visited_positions.len()
}

fn are_touching(a: &Position, b: &Position) -> bool {
    let same = a == b;
    let row_touch = a.y == b.y && a.x.abs_diff(b.x) == 1;
    let col_touch = a.x == b.x && a.y.abs_diff(b.y) == 1;
    let dia_touch = a.x.abs_diff(b.x) == 1 && a.y.abs_diff(b.y) == 1;

    same || row_touch || col_touch || dia_touch
}

fn move_up(position: Position) -> Position {
    Position {
        y: position.y + 1,
        ..position
    }
}
fn move_down(position: Position) -> Position {
    Position {
        y: position.y - 1,
        ..position
    }
}
fn move_right(position: Position) -> Position {
    Position {
        x: position.x + 1,
        ..position
    }
}
fn move_left(position: Position) -> Position {
    Position {
        x: position.x - 1,
        ..position
    }
}

fn parse_instructions(lines: Lines) -> Vec<Instruction> {
    lines
        .flat_map(|l| {
            let steps = l[2..].parse::<usize>().expect("cannot parse steps");

            let instr = l
                .chars()
                .next()
                .and_then(|c| match c {
                    'U' => Some(Instruction::Up),
                    'D' => Some(Instruction::Down),
                    'L' => Some(Instruction::Left),
                    'R' => Some(Instruction::Right),
                    _ => None,
                })
                .expect("unknown instruction");

            iter::repeat(instr).take(steps)
        })
        .collect()
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Up,
    Down,
    Left,
    Right,
}
