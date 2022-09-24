pub fn run() {
    let size = 32;
    let lines: Vec<Vec<u32>> = std::fs::read_to_string("data/3.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| char::to_digit(c, 2).unwrap())
                .collect()
        })
        .collect();

    let digits = lines[0].len();
    let half = lines.len() as u32 / 2;
    let mut sums = vec![0; digits];

    lines.iter().for_each(|line| {
        for i in 0..digits {
            sums[i] += line[i]
        }
    });

    let gamma: u32 = sums
        .into_iter()
        .map(|d| if d > half { 1 } else { 0 })
        .fold(0, |acc, digit| (acc << 1) + digit);

    println!("Part 1: {}", gamma * (!gamma << size - digits >> size - digits));
    println!("Part 2: {}", part_two());
}

fn part_two() -> usize {
    todo!();
}
