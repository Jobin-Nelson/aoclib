fn main() {
    let input = std::fs::read_to_string("data/day01.txt").unwrap();

    let mut all_food = input
        .split("\n\n")
        .map(|e| {
            e.lines()
                .map(|l| l.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect::<Vec<_>>();

    all_food.sort_by(|a, b| b.cmp(a));

    let part_1 = all_food.first().unwrap();

    let part_2: usize = all_food.iter().take(3).sum();

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
