use std::str::FromStr;

struct HandPair1(usize);
struct HandPair2(usize);

impl FromStr for HandPair1 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((o, p)) = s.split_once(' ') {
            let o = to_number1(o);
            let p = to_number1(p);
            let score = p + WIN_LOSE_1[(2 + o + p) % WIN_LOSE_1.len()];
            Ok(Self(score))
        } else {
            Err("Cannot be parsed".to_string())
        }
    }
}

impl FromStr for HandPair2 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((o, p)) = s.split_once(' ') {
            let o = to_number2(o);
            let p = to_number2(p);
            let score = CHOICE_VALUE[(o + p) % CHOICE_VALUE.len()] + WIN_LOSE_2[p];
            Ok(Self(score))
        } else {
            Err("Cannot be parsed".to_string())
        }
    }
}

const WIN_LOSE_1: [usize; 3] = [3, 6, 0];
const WIN_LOSE_2: [usize; 3] = [0, 3, 6];
const CHOICE_VALUE: [usize; 3] = [3, 1, 2];

fn to_number1(c: &str) -> usize {
    match c {
        "A" => 0,
        "B" => 2,
        "C" => 1,
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => unreachable!("Nice input guy"),
    }
}

fn to_number2(c: &str) -> usize {
    match c {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        "X" => 0,
        "Y" => 1,
        "Z" => 2,
        _ => unreachable!("Nice input guy"),
    }
}

fn main() {
    let input = std::fs::read_to_string("data/day02.txt").unwrap();

    let rounds = input.lines();

    let result: (usize, usize) = rounds
        .map(|l| {
            (
                l.parse::<HandPair1>().unwrap().0,
                l.parse::<HandPair2>().unwrap().0,
            )
        })
        .fold((0, 0), |(one, two), acc| (one + acc.0, two + acc.1));

    println!("Part 1: {}", result.0);
    println!("Part 2: {}", result.1);

    // let rules: HashMap<&str, usize> = HashMap::from([
    //     ("A X", 3),
    //     ("A Y", 0),
    //     ("A Z", 6),
    //     ("B X", 6),
    //     ("B Y", 3),
    //     ("B Z", 0),
    //     ("C X", 0),
    //     ("C Y", 6),
    //     ("C Z", 3),
    // ]);
    //
    // let score = HashMap::from([
    //     (88, 1),
    //     (89, 2),
    //     (90, 3),
    // ]);
    //
    // let total_score: usize = rounds
    //     .map(|l| rules[&l] + score[&(l.as_bytes().last().unwrap())])
    //     .sum();
    //
    // println!("Part 1: {}", total_score);
}
