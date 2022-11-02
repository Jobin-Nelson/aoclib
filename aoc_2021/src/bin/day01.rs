pub fn main() {
    println!("Part 1: {}", run_part(2).unwrap());
    println!("Part 2: {}", run_part(4).unwrap());
}

fn run_part(size: usize) -> Result<usize, std::io::Error> {
    Ok(aoc_2021::read_one_per_line::<i32>("data/day01.txt")?
        .windows(size)
        .filter(|win| win[0] < win[size - 1])
        .collect::<Vec<_>>()
        .len())
}
