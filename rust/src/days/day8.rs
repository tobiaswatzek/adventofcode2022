use std::{fs, path::PathBuf};

pub fn solve(input_path: &PathBuf) -> (String, String) {
    let input = fs::read_to_string(input_path).expect("file must be readable");

    let tree_heights = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| {
                    c.to_digit(10)
                        .expect(format!("cannot parse {c} as number in line {l}").as_str())
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // collection[y][x]

    let mut visible_trees = Vec::new();

    for (y, row) in tree_heights.iter().enumerate() {
        for (x, &tree) in row.iter().enumerate() {
            // trees on the outside are always visible
            if y == 0 || y == tree_heights.len() - 1 || x == 0 || x == row.len() - 1 {
                visible_trees.push((y, x));
                continue;
            }

            let left_invisible = row[..x].iter().any(|&t| t >= tree);
            if !left_invisible {
                visible_trees.push((y, x));
                continue;
            }

            let right_invisible = row[x + 1..].iter().any(|&t| t >= tree);
            if !right_invisible {
                visible_trees.push((y, x));
                continue;
            }

            let top_invisible = tree_heights[..y].iter().map(|r| r[x]).any(|t| t >= tree);
            if !top_invisible {
                visible_trees.push((y, x));
                continue;
            }

            let bottom_invisible = tree_heights[y+1..].iter().map(|r| r[x]).any(|t| t >= tree);
            if !bottom_invisible {
                visible_trees.push((y, x));
                continue;
            }
        }
    }

    let part_one = visible_trees.len();

    (part_one.to_string(), String::new())
}
