use std::{collections::HashSet, fs, hash::Hash, iter, path::PathBuf, str::Lines};

pub fn solve(input_path: &PathBuf) -> (String, String) {
    let input = fs::read_to_string(input_path).expect("file must be readable");
    let instructions = parse_instructions(input.lines());

    let part_one = run_simulation(&instructions, 1);
    let part_two = run_simulation(&instructions, 9);

    (part_one.to_string(), part_two.to_string())
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Position {
    x: i32,
    y: i32,
}

fn run_simulation(instructions: &Vec<Instruction>, tail_length: usize) -> usize {
    let mut head = Position { x: 0, y: 0 };
    let mut tails: Vec<Position> = iter::repeat(Position { x: 0, y: 0 })
        .take(tail_length)
        .collect();

    let mut visited: Vec<HashSet<Position>> = tails.iter().map(|&p| HashSet::from([p])).collect();

    for instruction in instructions {
        head = move_head(&head, instruction);
        tails[0] = move_tail(&head, &tails[0]);
        visited[0].insert(tails[0]);

        for i in 1..tail_length {
            let previous_tail = tails[i - 1];
            tails[i] = move_tail(&previous_tail, &tails[i]);
            visited[i].insert(tails[i]);
        }
    }

    /*let all_visited: HashSet<Position> = visited.iter().fold(HashSet::new(), |mut acc, item| {
        acc.extend(item);
        acc
    });*/

    //print_board(visited.last().unwrap());

    // let lens: Vec<usize> = visited.iter().map(|l| l.len()).collect();
    // println!("{lens:?}");

    visited.last().unwrap().len()
}

fn print_board(visited: &HashSet<Position>) {
    let min_x = visited.iter().map(|p| p.x).min().unwrap();
    let max_x = visited.iter().map(|p| p.x).max().unwrap();
    let min_y = visited.iter().map(|p| p.y).min().unwrap();
    let max_y = visited.iter().map(|p| p.y).max().unwrap();
    let padding = 2;
    println!("");

    for y in (min_y - padding..=max_y + padding).rev() {
        for x in min_x - padding..=max_x + padding {
            if x == 0 && y == 0 {
                print!("s");
            } else if visited.contains(&Position { x: x, y: y }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("");
}

fn move_head(&head: &Position, instruction: &Instruction) -> Position {
    match instruction {
        Instruction::Up => move_up(&head),
        Instruction::Down => move_down(&head),
        Instruction::Left => move_left(&head),
        Instruction::Right => move_right(&head),
    }
}

fn move_tail(&head: &Position, &tail: &Position) -> Position {
    if are_touching(&head, &tail) {
        return tail;
    }

    if head.x == tail.x {
        return Position {
            y: if head.y > tail.y {
                tail.y + 1
            } else {
                tail.y - 1
            },
            ..tail
        };
    }

    if head.y == tail.y {
        return Position {
            x: if head.x > tail.x {
                tail.x + 1
            } else {
                tail.x - 1
            },
            ..tail
        };
    }

    let diff_x = tail.x.abs_diff(head.x);
    let diff_y = tail.y.abs_diff(head.y);

    match (diff_x, diff_y) {
        (2, 2) => Position {
            x: if head.x == tail.x + 2 {
                head.x - 1
            } else {
                head.x + 1
            },
            y: if head.y == tail.y + 2 {
                head.y - 1
            } else {
                head.y + 1
            },
        },
        (1, _) => Position {
            x: head.x,
            y: if head.y == tail.y + 2 {
                head.y - 1
            } else {
                head.y + 1
            },
        },
        (_, 1) => Position {
            y: head.y,
            x: if head.x == tail.x + 2 {
                head.x - 1
            } else {
                head.x + 1
            },
        },
        _ => panic!("unexpected move dx {diff_x} dy {diff_y}"),
    }
}

fn are_touching(a: &Position, b: &Position) -> bool {
    let same = a == b;
    let row_touch = a.y == b.y && a.x.abs_diff(b.x) == 1;
    let col_touch = a.x == b.x && a.y.abs_diff(b.y) == 1;
    let dia_touch = a.x.abs_diff(b.x) == 1 && a.y.abs_diff(b.y) == 1;

    same || row_touch || col_touch || dia_touch
}

fn move_up(&position: &Position) -> Position {
    Position {
        y: position.y + 1,
        ..position
    }
}
fn move_down(&position: &Position) -> Position {
    Position {
        y: position.y - 1,
        ..position
    }
}
fn move_right(&position: &Position) -> Position {
    Position {
        x: position.x + 1,
        ..position
    }
}
fn move_left(&position: &Position) -> Position {
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
