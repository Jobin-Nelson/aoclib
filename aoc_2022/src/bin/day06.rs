use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("data/day06.txt").unwrap();
    let data_stream: Vec<char> = input.chars().collect();

    println!("Part 1: {}", find_marker(&data_stream, 4).unwrap());
    println!("Part 2: {}", find_marker(&data_stream, 14).unwrap());
}

fn find_marker(stream: &[char], distinct_num: usize) -> Option<usize> {
    for (i, w) in stream.windows(distinct_num).enumerate() {
        let unique: HashSet<&char> = HashSet::from_iter(w);
        if unique.len() == distinct_num {
            return Some(i + distinct_num);
        }
    }
    None
}
