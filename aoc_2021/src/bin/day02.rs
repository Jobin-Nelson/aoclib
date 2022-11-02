pub fn main() {
    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());
}

fn part_1() -> i32 {
    let input = std::fs::read_to_string("data/day02.txt").unwrap();
    let (horizontal, depth) = input.split("\n").fold((0, 0), |acc, n| {
        if let Some((direction, magnitude)) = n.split_once(' ') {
            let magnitude = magnitude.parse::<i32>().unwrap();
            match direction {
                "forward" => (acc.0 + magnitude, acc.1),
                "down" => (acc.0, acc.1 + magnitude),
                "up" => (acc.0, acc.1 - magnitude),
                _ => unreachable!(),
            }
        } else {
            acc
        }
    });
    horizontal * depth
}

fn part_2() -> i32 {
    let input = std::fs::read_to_string("data/day02.txt").unwrap();
    let (horizontal, depth, _) = input.split('\n').fold((0, 0, 0), |acc, n| {
        if let Some((direction, magnitude)) = n.split_once(' ') {
            let magnitude = magnitude.parse::<i32>().unwrap();
            match direction {
                "forward" => (acc.0 + magnitude, acc.1 + acc.2 * magnitude, acc.2),
                "down" => (acc.0, acc.1, acc.2 + magnitude),
                "up" => (acc.0, acc.1, acc.2 - magnitude),
                _ => unreachable!(),
            }
        } else {
            acc
        }
    });
    horizontal * depth
}
