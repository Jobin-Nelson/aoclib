use std::ops::Add;
use std::str::FromStr;

#[derive(Debug)]
enum Colors {
    Red(usize),
    Blue(usize),
    Green(usize),
}
#[derive(Debug)]
struct ParseColorError;

impl FromStr for Colors {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((num, color)) = s.trim().split_once(' ') {
            let n = num.parse::<usize>().unwrap();
            let color = match color {
                "red" => Colors::Red(n),
                "green" => Colors::Green(n),
                "blue" => Colors::Blue(n),
                _ => return Err(ParseColorError),
            };
            return Ok(color);
        }
        Err(ParseColorError)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Set {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug)]
struct ParseSetError;

impl FromStr for Set {
    type Err = ParseSetError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for color in s.trim().split(',') {
            let c = color.parse::<Colors>().unwrap();
            match c {
                Colors::Red(n) => red = n,
                Colors::Green(n) => green = n,
                Colors::Blue(n) => blue = n,
            }
        }
        Ok(Self { red, green, blue })
    }
}

impl Add for Set {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let red = self.red + rhs.red;
        let green = self.green + rhs.green;
        let blue = self.blue + rhs.blue;
        Self { red, green, blue }
    }
}

impl PartialOrd for Set {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.red.partial_cmp(&other.red) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.green.partial_cmp(&other.green) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.blue.partial_cmp(&other.blue)
    }
    fn lt(&self, other: &Self) -> bool {
        if self.red > other.red || self.green > other.green || self.blue > other.blue {
            return false;
        }
        true
        // matches!(self.partial_cmp(other), Some(Less))
    }
}

#[derive(Debug)]
struct Game {
    no: usize,
    sets: Vec<Set>,
}

#[derive(Debug)]
struct ParseGameError;

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((game_no, sets)) = s.trim().split_once(':') {
            let no = game_no
                .strip_prefix("Game ")
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let sets = sets
                .split(';')
                .map(|s| s.parse::<Set>().unwrap())
                .collect::<Vec<Set>>();
            return Ok(Self { no, sets });
        }
        Err(ParseGameError)
    }
}

fn main() {
    let input = std::fs::read_to_string("data/day02.txt").unwrap();
    let actual_set = Set {
        red: 12,
        green: 13,
        blue: 14,
    };
    println!("Part 1: {}", part_1(&input, actual_set));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str, actual: Set) -> usize {
    let games = input
        .lines()
        .map(|l| l.parse::<Game>().unwrap())
        .collect::<Vec<Game>>();

    games
        .iter()
        .filter_map(|g| {
            if g.sets.iter().all(|s| s < &actual) {
                return Some(g.no);
            };
            None
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    let games = input
        .lines()
        .map(|l| l.parse::<Game>().unwrap())
        .collect::<Vec<Game>>();
    games
        .iter()
        .map(|g| {
            let s = g.sets.iter().fold((0, 0, 0), |a, v| {
                (
                    std::cmp::max(a.0, v.red),
                    std::cmp::max(a.1, v.green),
                    std::cmp::max(a.2, v.blue),
                )
            });
            s.0 * s.1 * s.2
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let actual_set = Set {
            red: 12,
            green: 13,
            blue: 14,
        };
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        let expected = 8;
        let actual = part_1(&input, actual_set);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_2() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        let expected = 2286;
        let actual = part_2(&input);
        assert_eq!(expected, actual);
    }
}
