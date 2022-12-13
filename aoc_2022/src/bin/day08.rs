use std::collections::HashSet;

const UP: (i32, i32) = (-1, 0);
const DOWN: (i32, i32) = (1, 0);
const LEFT: (i32, i32) = (0, -1);
const RIGHT: (i32, i32) = (0, 1);

#[derive(Debug)]
struct Forest {
    width: i32,
    height: i32,
    grid: Vec<Vec<u8>>,
}

fn main() {
    let input = std::fs::read_to_string("data/day08.txt").unwrap();

    let mut grid = Vec::new();

    for line in input.lines() {
        let row: Vec<u8> = line.as_bytes().iter().map(|c| c - b'0').collect();
        grid.push(row)
    }

    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    let forest = Forest {
        width,
        height,
        grid,
    };

    println!("Part 1: {}", part_1(&forest));
    println!("Part 2: {}", part_2(&forest));
}

fn part_1(forest: &Forest) -> usize {
    let mut visited = HashSet::new();

    for (start, step, search) in [
        ((0, 0), RIGHT, DOWN),
        ((0, 0), DOWN, RIGHT),
        ((forest.height - 1, forest.width - 1), UP, LEFT),
        ((forest.height - 1, forest.width - 1), LEFT, UP),
    ] {
        let mut walk = start;

        while walk.0 >= 0 && walk.0 < forest.height && walk.1 >= 0 && walk.1 < forest.width {
            let (mut row, mut col) = walk;
            let mut tallest = forest.grid[row as usize][col as usize];

            visited.insert((row, col));

            while tallest < 9 {
                row += search.0;
                col += search.1;

                if row < 0 || row >= forest.height || col < 0 || col >= forest.width {
                    break;
                }

                let tree = forest.grid[row as usize][col as usize];

                if tree > tallest {
                    visited.insert((row, col));
                    tallest = tree;
                }
            }
            walk.0 += step.0;
            walk.1 += step.1;
        }
    }

    visited.len()
}

fn part_2(forest: &Forest) -> usize {
    let mut max_score = 0;

    for row in 0..forest.height {
        for col in 0..forest.width {
            let mut score = 1;
            for dir in [UP, DOWN, LEFT, RIGHT] {
                let mut walk = (row, col);
                let cur_height = forest.grid[walk.0 as usize][walk.1 as usize];
                walk.0 += dir.0;
                walk.1 += dir.1;

                let mut count = 0;

                while walk.0 >= 0 && walk.0 < forest.height && walk.1 >= 0 && walk.1 < forest.width
                {
                    count += 1;

                    if forest.grid[walk.0 as usize][walk.1 as usize] >= cur_height {
                        break;
                    }

                    walk.0 += dir.0;
                    walk.1 += dir.1;
                }

                score *= count;
            }

            max_score = max_score.max(score);
        }
    }

    max_score
}

// fn explore(forest: &Forest, row: i32, col: i32, origin: u8, visited: &mut HashSet<(i32, i32)>) -> usize {
//     if row < 0
//         || row >= forest.height as i32
//         || col < 0
//         || col >= forest.width as i32
//         || visited.contains(&(row, col))
//         || forest.grid[row as usize][col as usize] >= origin
//     {
//         return 0;
//     }
//
//     visited.insert((row, col));
//     let mut score = 0;
//     for dir in [UP, DOWN, RIGHT, LEFT] {
//         score += explore(
//             forest,
//             row + dir.0,
//             col + dir.1,
//             origin,
//             visited,
//         );
//     }
//     score
// }
