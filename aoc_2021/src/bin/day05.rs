use std::{collections::HashMap, num::ParseIntError, ops::RangeInclusive, str::FromStr};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position(u32, u32);

impl FromStr for Position {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(",").unwrap();
        Ok(Position(x.parse()?, y.parse()?))
    }
}

#[derive(Debug)]
struct Line {
    start: Position,
    end: Position,
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(" -> ").unwrap();
        Ok(Line {
            start: left.parse()?,
            end: right.parse()?,
        })
    }
}

fn main() {
    let lines = aoc_2021::read_one_per_line("data/day05.txt").unwrap();

    println!("Part 1: {}", get_dangerous_counts(&lines, false));
    println!("Part 2: {}", get_dangerous_counts(&lines, true));
}

fn get_dangerous_counts(lines: &Vec<Line>, diagonal: bool) -> u32 {
    let mut counts: HashMap<Position, u32> = HashMap::new();

    lines.iter().for_each(|item| {
        for pos in betwixt(&item.start, &item.end, diagonal) {
            counts.insert(pos, counts.get(&pos).unwrap_or(&0) + 1);
        }
    });

    counts
        .iter()
        .fold(0, |acc, (_, &count)| acc + if count > 1 { 1 } else { 0 })
}

fn betwixt(start: &Position, end: &Position, diagonal: bool) -> Vec<Position> {
    if start.0 != end.0 && start.1 != end.1 {
        if !diagonal {
            Vec::new()
        } else {
            steps(start.0, end.0)
                .zip(steps(start.1, end.1))
                .map(|(x, y)| Position(x, y))
                .collect()
        }
    } else {
        steps(start.0, end.0)
            .flat_map(|x| steps(start.1, end.1).map(move |y| Position(x, y)))
            .collect()
    }
}

fn steps(start: u32, end: u32) -> Box<dyn Iterator<Item = u32>> {
    if start < end {
        Box::new(RangeInclusive::new(start, end))
    } else {
        Box::new(RangeInclusive::new(end, start).rev())
    }
}
