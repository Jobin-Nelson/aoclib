use std::{
    cmp::Ordering,
    str::{Chars, FromStr},
};

#[derive(Debug, Eq, PartialEq, Clone)]
enum Val {
    Num(i32),
    List(Vec<Val>),
}

impl FromStr for Val {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut citer = s.chars();
        if citer.next().unwrap() != '[' {
            return Err("Bad input".to_string());
        }
        Ok(Self::parse_into(&mut citer))
    }
}

impl Val {
    fn parse_into(c: &mut Chars) -> Self {
        let mut result = Vec::new();
        let mut num = -1;
        while let Some(ch) = c.next() {
            match ch {
                '[' => result.push(Self::parse_into(c)),
                ',' => {
                    if num >= 0 {
                        result.push(Self::Num(num));
                        num = -1;
                    }
                }
                ']' => {
                    if num >= 0 {
                        result.push(Self::Num(num));
                    }
                    return Self::List(result);
                }
                '0'..='9' => {
                    if num == -1 {
                        num = (ch as u8 - b'0') as i32;
                    } else {
                        num = (num * 10) + (ch as u8 - b'0') as i32;
                    }
                }
                _ => unimplemented!(),
            }
        }
        Self::List(result)
    }

    fn compare(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Val::List(left), Val::List(right)) => {
                let mut idx = 0;
                while idx < left.len() && idx < right.len() {
                    match (&left[idx], &right[idx]) {
                        (Val::Num(l), Val::Num(r)) => {
                            if l != r {
                                return l.cmp(r);
                            }
                        }
                        (Val::List(_), Val::Num(r)) => {
                            let check = left[idx].compare(&Val::List(vec![Val::Num(*r)]));
                            if check != Ordering::Equal {
                                return check;
                            }
                        }
                        (Val::Num(l), Val::List(_)) => {
                            let check = Val::List(vec![Val::Num(*l)]).compare(&right[idx]);
                            if check != Ordering::Equal {
                                return check;
                            }
                        }
                        (Val::List(_), Val::List(_)) => {
                            let check = left[idx].compare(&right[idx]);
                            if check != Ordering::Equal {
                                return check;
                            }
                        }
                    }
                    idx += 1;
                }
                left.len().cmp(&right.len())
            }
            _ => unimplemented!(),
        }
    }
}

impl PartialOrd for Val {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Ord for Val {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    let input = std::fs::read_to_string("data/day13.txt").unwrap();

    let pair_list: Vec<Val> = input
        .split("\n\n")
        .flat_map(|c| c.lines().map(|l| l.parse::<Val>().unwrap()))
        .collect();

    println!("Part 1: {}", part_1(&pair_list));
    println!("Part 2: {}", part_2(&pair_list));
}

fn part_1(pairs: &[Val]) -> usize {
    let mut sum = 0;
    for (index, pair) in pairs.chunks(2).enumerate() {
        if pair[0] < pair[1] {
            sum += index + 1;
        }
    }
    sum
}

fn part_2(pairs: &[Val]) -> usize {
    let d2: Val = "[[2]]".parse().unwrap();
    let d6: Val = "[[6]]".parse().unwrap();

    let mut list = vec![d2.clone(), d6.clone()];

    list.extend(pairs.iter().cloned());

    list.sort();

    list.into_iter()
        .enumerate()
        .filter(|(_, p)| *p == d2 || *p == d6)
        .fold(1, |a, b| a * (b.0 + 1))
}
