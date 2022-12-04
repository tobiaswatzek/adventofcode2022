use std::{collections::HashMap, path::PathBuf};

mod day1;

pub fn solve_day(day: &u8, input_path: &PathBuf) -> (String, String) {
    let mut day_registry = HashMap::new();
    day_registry.insert(1, day1::solve);

    let solve = day_registry
        .get(day)
        .expect("solution for given day is not implemented");

    solve(input_path)
}
