use std::collections::VecDeque;

fn main() {
    let input = std::fs::read_to_string("data/day06.txt").unwrap();
    let fishes: Vec<usize> = input
        .trim_end()
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect();
    println!("Part 1: {}", get_count(&fishes, 80));
    println!("Part 2: {}", get_count(&fishes, 256));
}

fn get_count(fishes: &[usize], days: u32) -> usize {
    let mut count = VecDeque::from(vec![0; 9]);
    fishes.iter().for_each(|i| count[*i] += 1);

    for _ in 0..days {
        let new_fishes = count.pop_front().unwrap();
        count[6] += new_fishes;
        count.push_back(new_fishes);
    }
    count.iter().sum()
}
