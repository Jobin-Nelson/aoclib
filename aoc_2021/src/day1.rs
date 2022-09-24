pub fn run() {
    println!("Part 1: {}", run_part(2).unwrap());
    println!("Part 2: {}", run_part(4).unwrap());
}

fn run_part(size: usize) -> Result<usize, std::io::Error> {
    Ok(crate::read_one_per_line::<i32>("data/1.txt")?
        .windows(size)
        .filter(|win| win[0] < win[size - 1])
        .collect::<Vec<_>>()
        .len())
}
