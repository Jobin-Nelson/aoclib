use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("data/day10.txt").unwrap();

    let legal_pairs = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
    let syntax_score_table = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let autocomplete_score_table = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);
    let mut stack: Vec<char> = Vec::new();
    let mut syntax_score = 0;
    let mut autocomplete_scores = Vec::new();
    let mut autocomplete_score: u64 = 0;

    for line in input.lines() {
        for c in line.chars() {
            if legal_pairs.contains_key(&c) {
                if !stack.is_empty() && stack[stack.len() - 1] == legal_pairs[&c] {
                    stack.pop();
                } else {
                    syntax_score += syntax_score_table[&c];
                    stack.clear();
                    break;
                }
            } else {
                stack.push(c);
            }
        }
        while let Some(c) = stack.pop() {
            autocomplete_score = autocomplete_score * 5 + autocomplete_score_table[&c];
        }
        if autocomplete_score > 0 {
            autocomplete_scores.push(autocomplete_score);
        }
        autocomplete_score = 0;
    }
    autocomplete_scores.sort();

    println!("Part 1: {}", syntax_score);
    println!(
        "Part 2: {}",
        autocomplete_scores[autocomplete_scores.len() / 2]
    );
}
