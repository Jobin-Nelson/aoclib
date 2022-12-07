use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("data/day03.txt").unwrap();

    let lines: Vec<String> = input.lines().map(|l| l.to_string()).collect();

    println!("Part 1: {}", get_sum(&lines));
    println!("Part 2: {}", get_group_sum(&lines));
}

fn get_sum(input: &[String]) -> usize {
    input
        .iter()
        .map(|l| {
            let half = l.len() / 2;
            let (lower, upper) = l.split_at(half);
            let has: HashSet<char> = lower.chars().collect();
            for c in upper.chars() {
                if has.contains(&c) {
                    if c.is_ascii_lowercase() {
                        return (c as u8 - b'a' + 1) as usize;
                    } else {
                        return (c as u8 - b'A' + 27) as usize;
                    }
                }
            }

            0
        })
        .sum()
}

fn get_group_sum(input: &[String]) -> usize {
    input
        .chunks(3)
        .map(|group| {
            let mut group_iter = group.iter();
            let first: HashSet<char> = group_iter.next().unwrap().chars().collect();
            let second: HashSet<char> = group_iter.next().unwrap().chars().collect();
            for c in group_iter.next().unwrap().chars() {
                if first.contains(&c) && second.contains(&c) {
                    if c.is_ascii_lowercase() {
                        return (c as u8 - b'a' + 1) as usize;
                    } else {
                        return (c as u8 - b'A' + 27) as usize;
                    }
                }
            }
            0
        })
        .sum()
}
