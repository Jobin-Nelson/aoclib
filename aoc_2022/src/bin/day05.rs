use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct CrateStack {
    stacks: Vec<VecDeque<char>>,
}

impl CrateStack {
    pub fn new(stacks: Vec<VecDeque<char>>) -> Self {
        Self { stacks }
    }
}

fn main() {
    let input = std::fs::read_to_string("data/day05.txt").unwrap();
    let mut lines = input.lines();

    let mut ship = CrateStack::new(Vec::new());

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        for pair in line
            .chars()
            .enumerate()
            .filter(|(i, c)| c != &' ' && (i + 3) % 4 == 0)
        {
            let stack = (pair.0 - 1) / 4;
            while ship.stacks.len() < stack + 1 {
                ship.stacks.push(VecDeque::new());
            }
            ship.stacks[stack].push_front(pair.1);
        }
    }

    for i in 0..ship.stacks.len() {
        ship.stacks[i].pop_front();
    }

    let ship_2 = ship.clone();

    let instructions: Vec<String> = lines.map(|l| l.to_string()).collect();

    println!("Part 1: {}", part_1(&instructions, ship));
    println!("Part 2: {}", part_2(&instructions, ship_2));
}

fn part_1(lines: &[String], mut ship: CrateStack) -> String {
    for line in lines {
        let mut iter = line.split_whitespace();
        iter.next();
        let amount: usize = iter.next().unwrap().parse().unwrap();
        iter.next();
        let from: usize = iter.next().unwrap().parse().unwrap();
        iter.next();
        let to: usize = iter.next().unwrap().parse().unwrap();

        for _ in 0..amount {
            let moved_item = ship.stacks[from - 1].pop_back().unwrap();
            ship.stacks[to - 1].push_back(moved_item);
        }
    }

    output(&ship)
}

fn part_2(lines: &[String], mut ship: CrateStack) -> String {
    for line in lines {
        let mut iter = line.split_whitespace();
        iter.next();
        let amount: usize = iter.next().unwrap().parse().unwrap();
        iter.next();
        let from: usize = iter.next().unwrap().parse().unwrap();
        iter.next();
        let to: usize = iter.next().unwrap().parse().unwrap();

        let split_point = ship.stacks[from - 1].len() - amount;
        let mut moved_crates = ship.stacks[from - 1].split_off(split_point);
        ship.stacks[to - 1].append(&mut moved_crates);
    }

    output(&ship)
}

fn output(ship: &CrateStack) -> String {
    let mut res = String::new();
    for i in 0..ship.stacks.len() {
        res.push(*ship.stacks[i].back().unwrap());
    }
    res
}
