use std::{collections::HashMap, path::PathBuf};

mod day1;
mod day3;

pub fn solve_day(day: &u8, input_path: &PathBuf) -> (String, String) {
    let mut day_registry: HashMap<u8, fn (&PathBuf) -> (String, String)> = HashMap::new();
    day_registry.insert(1, day1::solve);
    day_registry.insert(3, day3::solve);

    let solve = day_registry
        .get(day)
        .expect("solution for given day is not implemented");

    solve(input_path)
}
