use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Debug, Default, Clone)]
struct Monkey {
    items: VecDeque<usize>,
    test: usize,
    op: Op,
    pass: usize,
    fail: usize,
    inspect_count: usize,
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut monkey = Self::default();

        for line in s.lines() {
            let words: Vec<&str> = line.trim().split(' ').collect();

            match words[0] {
                "Monkey" => monkey = Self::default(),
                "Starting" => {
                    let (_, items) = line.split_once(": ").unwrap();

                    monkey.items = items.split(", ").map(|n| n.parse().unwrap()).collect();
                }
                "Operation:" => {
                    monkey.op = match (words[4], words[5]) {
                        ("+", "old") => Op::AddSelf,
                        ("+", num) => Op::Add(num.parse().unwrap()),
                        ("*", "old") => Op::MulSelf,
                        ("*", num) => Op::Mul(num.parse().unwrap()),
                        (_, _) => return Err("Invalid operation found".to_string()),
                    }
                }
                "Test:" => monkey.test = words[3].parse().unwrap(),
                "If" => {
                    if words[1] == "true:" {
                        monkey.pass = words[5].parse().unwrap();
                    } else {
                        monkey.fail = words[5].parse().unwrap();
                    }
                }
                _ => return Err("No match arms satisfied".to_string()),
            }
        }

        Ok(monkey)
    }
}

#[derive(Debug, Default, Clone)]
enum Op {
    Add(usize),
    AddSelf,
    Mul(usize),
    MulSelf,
    #[default]
    Noop,
}

impl Op {
    fn calc(&self, worry: usize) -> usize {
        match self {
            Self::Add(n) => worry + n,
            Self::AddSelf => worry + worry,
            Self::Mul(n) => worry * n,
            Self::MulSelf => worry * worry,
            _ => unimplemented!(),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("data/day11.txt").unwrap();

    let mut monkeys_1: Vec<Monkey> = input.split("\n\n").map(|m| m.parse().unwrap()).collect();

    let mut monkeys_2 = monkeys_1.clone();

    // part 1
    for _ in 0..20 {
        round(&mut monkeys_1, 0);
    }

    let mut inspects: Vec<usize> = monkeys_1.iter().map(|m| m.inspect_count).collect();
    inspects.sort_by(|a, b| b.cmp(a));
    let monkey_business_1 = inspects[0] * inspects[1];

    // part 2
    let modval: usize = monkeys_2.iter().map(|m| m.test).product();

    for _ in 0..10000 {
        round(&mut monkeys_2, modval);
    }

    let mut inspects: Vec<usize> = monkeys_2.iter().map(|m| m.inspect_count).collect();
    inspects.sort_by(|a, b| b.cmp(a));
    let monkey_business_2 = inspects[0] * inspects[1];

    println!("Part 1: {}", monkey_business_1);
    println!("Part 2: {}", monkey_business_2);
}

fn round(mvec: &mut Vec<Monkey>, modval: usize) {
    for i in 0..mvec.len() {
        while let Some(item) = mvec[i].items.pop_front() {
            mvec[i].inspect_count += 1;
            let worry = if modval == 0 {
                mvec[i].op.calc(item) / 3
            } else {
                mvec[i].op.calc(item) % modval
            };
            let next_monkey = if worry % mvec[i].test == 0 {
                mvec[i].pass
            } else {
                mvec[i].fail
            };
            mvec[next_monkey].items.push_back(worry);
        }
    }
}
