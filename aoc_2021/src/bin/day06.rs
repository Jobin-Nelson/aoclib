use std::collections::VecDeque;

fn main() {
    let input: Vec<usize> = std::fs::read_to_string("data/day06.txt")
        .unwrap()
        .split(',')
        .filter_map(|i| i.parse().ok())
        .collect();
    println!("Part 1: {}", get_count(&input, 80));
    println!("Part 1: {}", get_count(&input, 256));
}

fn get_count(fishes: &Vec<usize>,days: u32) -> u64 {
    let mut count = VecDeque::from(vec![0; 9]);
    fishes.iter().for_each(|i| count[*i] += 1);
    
    for _ in 0..days {
        let new_fishes = count.pop_front().unwrap();
        count[6] += new_fishes;
        count.push_back(new_fishes);
    }
    count.iter().sum()
}
