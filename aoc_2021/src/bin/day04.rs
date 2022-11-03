use std::collections::HashSet;
use std::str::Lines;

#[derive(Debug, Clone)]
struct Board {
    sets: Vec<HashSet<u32>>,
}

impl Board {
    fn new(lines: &mut Lines) -> Option<Board> {
        let mut sets: Vec<HashSet<u32>> = Vec::new();

        let empty = lines.next()?;
        if empty != "" {
            panic!("Didn't encounter empty line");
        }

        let rows: Vec<Vec<u32>> = lines
            .take(5)
            .map(|l| {
                l.split_whitespace()
                    .map(|i| i.parse::<u32>().unwrap())
                    .collect()
            })
            .collect();

        for col in 0..5 {
            let mut set = HashSet::new();
            for row in 0..5 {
                set.insert(rows[row][col]);
            }
            sets.push(set);
        }

        for row in rows {
            sets.push(HashSet::from_iter(row));
        }

        Some(Board { sets })
    }

    fn turn(&mut self, m: u32) -> bool {
        let mut complete = false;

        for set in self.sets.iter_mut() {
            if set.remove(&m) {
                complete |= set.is_empty();
            }
        }
        complete
    }

    fn remaining_sum(&self) -> u32 {
        HashSet::<&u32>::from_iter(self.sets.iter().flatten())
            .into_iter()
            .sum()
    }
}

fn main() {
    let input = std::fs::read_to_string("data/day04.txt").unwrap();
    let mut lines = input.lines();

    let moves: Vec<u32> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|i| i.parse::<u32>().unwrap())
        .collect();

    let mut boards = Vec::new();

    while let Some(board) = Board::new(&mut lines) {
        boards.push(board)
    }

    // Part 1
    {
        let mut boards = boards.clone();
        'moves: for m in &moves {
            for board in boards.iter_mut() {
                if board.turn(*m) {
                    println!("Part 1: {}", m * board.remaining_sum());
                    break 'moves;
                }
            }
        }
    }

    // Part 2
    {
        let mut last_result = 0;
        for m in &moves {
            let mut to_remove = Vec::new();
            for (idx, board) in boards.iter_mut().enumerate() {
                if board.turn(*m) {
                    last_result = m * board.remaining_sum();
                    to_remove.push(idx);
                }
            }

            for idx in to_remove.iter().rev() {
                boards.remove(*idx);
            }
        }

        println!("Part 2: {}", last_result);
    }
}
