use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("data/day01.txt").unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|v| {
            if v.chars().all(|v| v.is_whitespace()) {
                return 0;
            }
            let mut iter = v.bytes().filter(|d| d.is_ascii_digit());
            let d1 = iter.next().unwrap();
            let d2 = iter.last().unwrap_or(d1);
            ((d1 - b'0') * 10 + (d2 - b'0')) as usize
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    let digits = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let mut res: usize = 0;
    for line in input.lines() {
        let mut d1 = 0;
        let mut d2 = 0;
        'first: for (i, c) in line.bytes().enumerate() {
            if c.is_ascii_digit() {
                d1 = c - b'0';
                break;
            }
            for digit in digits.clone().into_keys() {
                if line[i..].starts_with(digit) {
                    d1 = digits[digit];
                    break 'first;
                }
            }
        }
        'second: for (i, c) in (line.bytes().enumerate()).rev() {
            if c.is_ascii_digit() {
                d2 = c - b'0';
                break;
            }
            for digit in digits.clone().into_keys() {
                if line[i..].starts_with(digit) {
                    d2 = digits[digit];
                    break 'second;
                }
            }
        }
        res += ((d1 * 10) + d2) as usize
    }
    res
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let actual = part_2(input);
        assert_eq!(281, actual);
    }
}
