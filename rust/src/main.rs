use std::{env, error, fs, path::{PathBuf}};

fn main() {

    let args = parse_args().expect("arguments are expected");

    let (part_one, part_two) = day1(&args.file_path());

    println!("Day 1:\n\tPart one: {part_one}\n\tPart two: {part_two}");
}

#[derive(Debug)]
struct Arguments {
    data_dir: PathBuf,
    day: u8,
}

impl Arguments {
    fn file_path(&self) -> PathBuf {
        self.data_dir.join(format!("day{}.txt", self.day))
    }
}

fn parse_args() -> Result<Arguments, Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    let day = match args[1].parse::<u8>()? {
        d @ 1..=24 => d,
        d => return Err(format!("day must be between 1 and 24 but is {d}").into()),
    };

    let data_dir = match args[2].trim() {
        s if !s.is_empty() => PathBuf::from(s),
        _ => return Err("data dir must be passed as argument".into()),
    };

    Ok(Arguments { data_dir, day })
}

fn day1(input_path: &PathBuf) -> (String, String) {
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
