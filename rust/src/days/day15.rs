use std::{fs, path::PathBuf};

use regex::Regex;

pub fn solve(input_path: &PathBuf) -> (String, String) {
    let input = fs::read_to_string(input_path).unwrap();

    // Sensor at x=1363026, y=2928920: closest beacon is at x=1571469, y=3023534
    let line_regex =
        Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$")
            .unwrap();
    let pairs = input
        .lines()
        .filter_map(|l| {
            line_regex.captures(l).and_then(|c| {
                Some(Pair::new(
                    Point {
                        x: c[1].parse().unwrap(),
                        y: c[2].parse().unwrap(),
                    },
                    Point {
                        x: c[3].parse().unwrap(),
                        y: c[4].parse().unwrap(),
                    },
                ))
            })
        })
        .collect::<Vec<_>>();

    let max_distance = pairs.iter().map(|p| p.distance).max().unwrap();

    let min_x = pairs
        .iter()
        .flat_map(|p| [p.beacon.x, p.sensor.x])
        .min()
        .unwrap()
        - max_distance as i64;
    let max_x = pairs
        .iter()
        .flat_map(|p| [p.beacon.x, p.sensor.x])
        .max()
        .unwrap()
        + max_distance as i64;

    let y = 2_000_000;
    let c = (min_x..max_x)
        .filter(|&x| pairs.iter().any(|p| p.is_in_range(&Point { x, y })))
        .count();

    (c.to_string(), String::new())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pair {
    sensor: Point,
    beacon: Point,
    distance: u64,
}

impl Pair {
    pub fn new(sensor: Point, beacon: Point) -> Self {
        let distance = sensor.distance_to(&beacon);

        Self {
            sensor,
            beacon,
            distance,
        }
    }

    pub fn is_in_range(&self, other: &Point) -> bool {
        &self.beacon != other && self.sensor.distance_to(other) <= self.distance
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn distance_to(&self, other: &Point) -> u64 {
        ((self.x - other.x).abs() + (self.y - other.y).abs())
            .try_into()
            .unwrap()
    }
}
