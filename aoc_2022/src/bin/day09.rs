use std::collections::HashSet;

struct Rope {
    seg: Vec<(i32, i32)>,
    visited: HashSet<(i32, i32)>,
}

impl Rope {
    const DIR: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

    fn new(num: usize) -> Self {
        Self {
            seg: vec![(0, 0); num],
            visited: HashSet::new(),
        }
    }

    fn make_move(&mut self, dir: &Direction, dist: i32) {
        for _ in 0..dist {
            let delta = Self::DIR[*dir as usize];
            self.seg[0].0 += delta.0;
            self.seg[0].1 += delta.1;

            for i in 1..self.seg.len() {
                let row_diff = self.seg[i-1].0 - self.seg[i].0;
                let col_diff = self.seg[i-1].1 - self.seg[i].1;
            

                if row_diff.abs() > 1 || col_diff.abs() > 1 {
                    self.seg[i].0 += row_diff.signum();
                    self.seg[i].1 += col_diff.signum();
                }
            }

            self.visited.insert(self.seg[self.seg.len() - 1]);
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn parse(s: &str) -> Self {
        match s {
            "U" => Self::Up,
            "D" => Self::Down,
            "R" => Self::Right,
            "L" => Self::Left,
            _ => unimplemented!(),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("data/day09.txt").unwrap();

    let mut steps = Vec::new();

    for line in input.lines() {
        let (dir, dist) = line.split_once(' ').unwrap();
        let dir = Direction::parse(dir);
        let dist: i32 = dist.parse().unwrap();
        steps.push((dir, dist));
    }

    let mut rope_1 = Rope::new(2);
    let mut rope_2 = Rope::new(10);

    for (dir, dist) in steps {
        rope_1.make_move(&dir, dist);
        rope_2.make_move(&dir, dist);
    }

    println!("Part 1: {}", rope_1.visited.len());
    println!("Part 2: {}", rope_2.visited.len());
}
