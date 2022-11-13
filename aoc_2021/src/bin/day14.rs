use std::collections::HashMap;

type Pairs = HashMap<(char, char), usize>;
type Rules = HashMap<(char, char), char>;

fn main() {
    let input = std::fs::read_to_string("data/day14.txt").unwrap();
    let mut rules: Rules = HashMap::new();

    let mut lines = input.lines();
    let template: Vec<char> = lines.next().unwrap().chars().collect();
    let firstchar = template[0];

    let mut pairs: Pairs = HashMap::new();
    for w in template.windows(2) {
        pairs
            .entry((w[0], w[1]))
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }

    lines.next(); // skip blank line;

    for line in lines {
        if let Some((key, value)) = line.split_once(" -> ") {
            let mut chars = key.chars();
            let a = chars.next().unwrap();
            let b = chars.next().unwrap();
            rules.insert((a, b), value.chars().next().unwrap());
        };
    }

    pairs = run_steps(&pairs, &rules, 10);
    println!("Part 1: {}", get_counts(&pairs, &firstchar));

    pairs = run_steps(&pairs, &rules, 30);
    println!("Part 1: {}", get_counts(&pairs, &firstchar));
}

fn get_counts(pairs: &Pairs, firstchar: &char) -> usize {
    let mut count: HashMap<char, usize> = HashMap::new();
    count.insert(*firstchar, 1);

    for (key, value) in pairs {
        count
            .entry(key.1)
            .and_modify(|v| *v += value)
            .or_insert(*value);
    }

    let max = count.values().max().unwrap();
    let min = count.values().min().unwrap();

    max - min
}

fn run_steps(pairs: &Pairs, rules: &Rules, count: usize) -> Pairs {
    let mut pairs = pairs.clone();

    for _ in 0..count {
        let mut new_pairs = pairs.clone();

        for (key, value) in pairs {
            let insert = rules.get(&key).unwrap();
            new_pairs.entry(key).and_modify(|v| *v -= value);
            new_pairs
                .entry((key.0, *insert))
                .and_modify(|v| *v += value)
                .or_insert(value);
            new_pairs
                .entry((*insert, key.1))
                .and_modify(|v| *v += value)
                .or_insert(value);
        }
        pairs = new_pairs;
    }

    pairs
}
