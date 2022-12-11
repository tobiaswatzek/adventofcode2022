use std::{collections::VecDeque, fs, path::PathBuf};

pub fn solve(input_path: &PathBuf) -> (String, String) {
    let input = fs::read_to_string(input_path).unwrap();

    let part_one = solve_part_one(&input);
    let part_two = solve_part_two(&input);

    (part_one.to_string(), part_two.to_string())
}

fn solve_part_one(input: &str) -> u64 {
    let mut monkeys = parse_monkeys(&input);
    run_rounds(20, 3, &mut monkeys);

    let mut counts: Vec<u64> = monkeys.iter().map(|m| m.inspection_count).collect();
    counts.sort();

    counts.iter().rev().take(2).product()
}

fn solve_part_two(input: &str) -> u64 {
    let mut monkeys = parse_monkeys(&input);
    run_rounds(10_000, 1, &mut monkeys);

    let mut counts: Vec<u64> = monkeys.iter().map(|m| m.inspection_count).collect();
    counts.sort();

    counts.iter().rev().take(2).product()
}

fn run_rounds(number: usize, worry_divisor: u64, monkeys: &mut Vec<Monkey>) {
    for _ in 0..number {
        run_round(monkeys, worry_divisor);
    }

    fn run_round(monkeys: &mut Vec<Monkey>, worry_divisor: u64) {
        for i in 0..monkeys.len() {
            monkeys[i].inspect_items(worry_divisor);
            while let Some(throw) = monkeys[i].perform_throw() {
                monkeys[throw.monkey as usize].catch_item(throw.item);
            }
        }
    }
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    struct Params {
        number: u8,
        operation: Operation,
        test: Test,
        items: VecDeque<u64>,
    }

    let params = input
        .split("\n\n")
        .map(|part| {
            let lines: Vec<&str> = part.lines().collect();

            let number = lines[0]
                .split(" ")
                .last()
                .unwrap()
                .trim_end_matches(':')
                .parse::<u8>()
                .unwrap();

            let items: VecDeque<u64> = lines[1]
                .split(": ")
                .last()
                .unwrap()
                .split(", ")
                .map(|s| s.parse::<u64>().unwrap())
                .collect();

            let raw_operation: Vec<&str> =
                lines[2].split("= ").last().unwrap().split(" ").collect();

            let operation = match raw_operation[..] {
                ["old", "*", "old"] => Operation::Square,
                ["old", "*", n] => Operation::Multiply(n.parse().unwrap()),
                ["old", "+", n] => Operation::Add(n.parse().unwrap()),
                _ => panic!("cannot parse operation {raw_operation:?}"),
            };

            let divisible_by = lines[3]
                .split("by ")
                .last()
                .unwrap()
                .parse::<u64>()
                .unwrap();

            let if_true = lines[4]
                .split("monkey ")
                .last()
                .unwrap()
                .parse::<u8>()
                .unwrap();

            let if_false = lines[5]
                .split("monkey ")
                .last()
                .unwrap()
                .parse::<u8>()
                .unwrap();

            Params {
                number,
                operation,
                test: Test::new(divisible_by, if_true, if_false),
                items,
            }
        })
        .collect::<Vec<_>>();

    let mod_base = params.iter().map(|p| p.test.divisor).product();

    params
        .iter()
        .map(|p| Monkey::new(p.number, p.operation, p.test, mod_base, p.items.clone()))
        .collect()
}

#[derive(Clone, Copy, Debug)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

#[derive(Clone, Copy, Debug)]
struct Test {
    divisor: u64,
    if_true: u8,
    if_false: u8,
}

impl Test {
    pub fn new(divisor: u64, if_true: u8, if_false: u8) -> Self {
        Test {
            divisor,
            if_true,
            if_false,
        }
    }

    pub fn run(&self, candidate: u64) -> u8 {
        match candidate % self.divisor {
            0 => self.if_true,
            _ => self.if_false,
        }
    }
}

#[derive(Debug)]
struct Throw {
    monkey: u8,
    item: u64,
}

impl Throw {
    pub fn new(monkey: u8, item: u64) -> Self {
        Throw { monkey, item }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Monkey {
    number: u8,
    operation: Operation,
    test: Test,
    items: VecDeque<u64>,
    throws: VecDeque<Throw>,
    inspection_count: u64,
    mod_base: u64,
}

impl Monkey {
    pub fn new(
        number: u8,
        operation: Operation,
        test: Test,
        mod_base: u64,
        items: VecDeque<u64>,
    ) -> Self {
       // let mod_items = items.iter().map(|i| i % mod_base).collect();

        Monkey {
            number,
            operation,
            test,
            items: items,
            throws: VecDeque::new(),
            inspection_count: 0,
            mod_base,
        }
    }

    pub fn inspect_items(&mut self, worry_divisor: u64) {
        while let Some(item) = self.items.pop_front() {
            let mut new_item = match self.operation {
                Operation::Add(n) => item + n,
                Operation::Multiply(n) => item * n,
                Operation::Square => item * item,
            } % self.mod_base;

            new_item /= worry_divisor;
            // new_item %= self.mod_base;

            let destination_monkey = self.test.run(new_item);

            let throw = Throw::new(destination_monkey, new_item);

            self.throws.push_back(throw);
            self.inspection_count += 1;
        }
    }

    pub fn perform_throw(&mut self) -> Option<Throw> {
        self.throws.pop_front()
    }

    pub fn catch_item(&mut self, item: u64) {
        self.items.push_back(item)
    }
}
