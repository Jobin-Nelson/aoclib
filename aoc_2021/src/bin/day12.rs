use std::collections::{HashMap, HashSet};

type CavePath = HashMap<String, Vec<String>>;

#[derive(Debug, Default)]
struct VisitedTwice {
    visited: HashMap<String, i32>,
    path: Vec<String>,
    unique: HashSet<String>,
}

fn main() {
    let input = std::fs::read_to_string("data/day12.txt").unwrap();
    let mut caves: CavePath = HashMap::new();

    for line in input.lines() {
        let mut split = line.split('-');

        let from = split.next().unwrap();
        let to = split.next().unwrap();

        caves
            .entry(from.to_string())
            .or_default()
            .push(to.to_string());
        caves
            .entry(to.to_string())
            .or_default()
            .push(from.to_string());
    }

    let mut visited = HashSet::new();
    let mut visited_twice = VisitedTwice {
        ..Default::default()
    };

    for c in caves.keys() {
        if *c != "start" && *c != "end" {
            let first = c.chars().next().unwrap();
            if first.is_lowercase() {
                count_visited_twice("start", &mut visited_twice, &caves, &Some(c.to_string()));
            } else {
                count_visited_twice("start", &mut visited_twice, &caves, &None);
            }
        }
    }

    println!("Part 1: {}", count_visited("start", &mut visited, &caves));
    println!("Part 2: {:?}", visited_twice.unique.len());
}

fn count_visited(cur_cave: &str, visited: &mut HashSet<String>, caves: &CavePath) -> usize {
    if cur_cave == "end" {
        return 1;
    }
    let mut count = 0;

    if cur_cave.chars().next().unwrap().is_lowercase() {
        visited.insert(cur_cave.to_string());
    }

    for next_cave in caves.get(cur_cave).unwrap() {
        if !visited.contains(next_cave) {
            count += count_visited(next_cave, visited, caves);
        }
    }

    visited.remove(cur_cave);

    count
}

fn count_visited_twice(
    cur_cave: &str,
    visited_twice: &mut VisitedTwice,
    caves: &CavePath,
    double: &Option<String>,
) -> usize {
    if cur_cave == "end" {
        visited_twice
            .unique
            .insert(format!("{:?}", visited_twice.path));
        return 1;
    }

    visited_twice
        .visited
        .entry(cur_cave.to_string())
        .and_modify(|v| *v += 1)
        .or_insert(1);
    visited_twice.path.push(cur_cave.to_string());

    let mut count = 0;

    for next_cave in caves.get(cur_cave).unwrap() {
        if next_cave.chars().next().unwrap().is_uppercase()
            || *visited_twice
                .visited
                .entry(next_cave.to_string())
                .or_default()
                == 0
        {
            count += count_visited_twice(next_cave, visited_twice, caves, double);
        } else if let Some(d) = double {
            if d == next_cave
                && *visited_twice
                    .visited
                    .entry(next_cave.to_string())
                    .or_default()
                    < 2
            {
                count += count_visited_twice(next_cave, visited_twice, caves, double);
            }
        }
    }

    visited_twice
        .visited
        .entry(cur_cave.to_string())
        .and_modify(|v| *v -= 1);
    visited_twice.path.pop();

    count
}
