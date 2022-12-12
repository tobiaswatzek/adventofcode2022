use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    path::PathBuf,
};

pub fn solve(input_path: &PathBuf) -> (String, String) {
    let input = fs::read_to_string(input_path).unwrap();
    let height_map = parse_input(&input);

    let test = bfs(&height_map);

    return (test.to_string(), String::new());
}

fn bfs(height_map: &HeightMap) -> usize {
    let mut predecessors = HashMap::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(height_map.start);
    visited.insert(height_map.start);

    while let Some(point) = queue.pop_front() {
        if point == height_map.end {
            break;
        }

        for &child in height_map.nodes.get(&point).unwrap() {
            if !visited.contains(&child) {
                queue.push_back(child);
                visited.insert(child);
                predecessors.insert(child, point);
            }
        }
    }

    let mut key = height_map.end;
    let mut path = Vec::new();
    path.push(key);
    while let Some(&p) = predecessors.get(&key) {
        path.push(p);
        key = p;
    }

    path.len() - 1
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn neighbors(&self) -> [Option<Self>; 4] {
        [self.left(), self.top(), self.right(), self.bottom()]
    }

    pub fn left(&self) -> Option<Self> {
        if self.x == 0 {
            return None;
        }
        Some(Point {
            x: self.x - 1,
            y: self.y,
        })
    }

    pub fn right(&self) -> Option<Self> {
        Some(Point {
            x: self.x + 1,
            y: self.y,
        })
    }

    pub fn top(&self) -> Option<Self> {
        Some(Point {
            x: self.x,
            y: self.y + 1,
        })
    }

    pub fn bottom(&self) -> Option<Self> {
        if self.y == 0 {
            return None;
        }
        Some(Point {
            x: self.x,
            y: self.y - 1,
        })
    }
}

#[derive(Debug, Clone)]
struct HeightMap {
    start: Point,
    end: Point,
    nodes: HashMap<Point, Vec<Point>>,
}

#[derive(Debug, Clone, PartialEq)]
enum PointType {
    None,
    Start,
    End,
}

#[derive(Debug, Clone, PartialEq)]
struct PointValue(u32, PointType);

fn parse_input(input: &str) -> HeightMap {
    let point_value_grid = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'S' => PointValue('a' as u32, PointType::Start),
                    'E' => PointValue('z' as u32, PointType::End),
                    n => PointValue(n as u32, PointType::None),
                })
                .collect::<Vec<PointValue>>()
        })
        .collect::<Vec<Vec<PointValue>>>();

    let mut nodes: HashMap<Point, Vec<Point>> =
        HashMap::with_capacity(point_value_grid.len() * point_value_grid[0].len());
    let mut start: Option<Point> = None;
    let mut end: Option<Point> = None;

    for (y, row) in point_value_grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let point = Point { x, y };
            match cell.1 {
                PointType::Start => start = Some(point),
                PointType::End => end = Some(point),
                _ => (),
            };
            nodes.insert(point, build_nodes(&point, &point_value_grid));
        }
    }

    HeightMap {
        start: start.unwrap(),
        end: end.unwrap(),
        nodes,
    }
}

fn build_nodes(current: &Point, point_value_grid: &Vec<Vec<PointValue>>) -> Vec<Point> {
    let PointValue(current_weight, _) = point_value_grid[current.y][current.x];

    current
        .neighbors()
        .iter()
        .filter_map(|&neighbor| match neighbor {
            Some(p) => point_value_grid
                .get(p.y)
                .and_then(|row| row.get(p.x))
                .and_then(|&PointValue(w, _)| {
                    if w <= current_weight + 1 {
                        return Some(p);
                    }

                    None
                }),
            None => None,
        })
        .collect()
}
