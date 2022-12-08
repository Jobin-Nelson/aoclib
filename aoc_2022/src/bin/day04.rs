fn main() {
    let input = std::fs::read_to_string("data/day04.txt").unwrap();
    let lines = input.lines();

    let (part_1, part_2) = lines
        .map(|l| {
            let (first, second) = l.split_once(',').unwrap();
            let (a, b) = first.split_once('-').unwrap();
            let (c, d) = second.split_once('-').unwrap();
            let a: usize = a.parse().unwrap();
            let b: usize = b.parse().unwrap();
            let c: usize = c.parse().unwrap();
            let d: usize = d.parse().unwrap();
            let mut res = (0, 0);
            if (a <= c && d <= b) || (c <= a && b <= d) {
                res = (1, 1);
            } else if (a <= c && c <= b) || (a <= d && d <= b) {
                res = (0, 1)
            }
            res
        })
        .fold((0, 0), |acc, v| (acc.0 + v.0, acc.1 + v.1));

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
