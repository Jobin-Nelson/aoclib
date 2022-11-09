use std::collections::HashSet;

#[derive(Debug)]
struct Floor {
    grid: Vec<u8>,
    width: usize,
    height: usize,
}

impl Floor {
    fn get(&self, x: usize, y: usize) -> u8 {
        self.grid[y * self.width + x]
    }
    fn is_lowest(&self, x: usize, y: usize) -> bool {
        let surroundings = self.surrounding(x, y);
        let cur = self.grid[y * self.width + x];
        for s in surroundings {
            let idx = s.1 * self.width + s.0;
            if cur >= self.grid[idx] {
                return false;
            }
        }
        true
    }
    fn surrounding(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::new();

        if x > 0 {
            result.push((x - 1, y));
        }
        if x < self.width - 1 {
            result.push((x + 1, y));
        }
        if y > 0 {
            result.push((x, y - 1));
        }
        if y < self.height - 1 {
            result.push((x, y + 1));
        }
        result
    }
    fn basin_size(&self, x: usize, y: usize) -> usize {
        let mut hs: HashSet<(usize, usize)> = HashSet::new();
        let mut check = Vec::new();

        hs.insert((x, y));
        let mut x = x;
        let mut y = y;
        loop {
            let mut sur = self.surrounding(x, y);
            sur.retain(|x| !hs.contains(x));

            for s in sur {
                if self.get(s.0, s.1) != 9 {
                    check.push(s);
                    hs.insert(s);
                }
            }

            if let Some(next) = check.pop() {
                x = next.0;
                y = next.1;
            } else {
                return hs.len();
            }
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("data/day09.txt").unwrap();
    let lines: Vec<String> = input.lines().map(|l| l.to_string()).collect();

    let width = lines[0].len();
    let height = lines.len();
    let mut grid: Vec<u8> = Vec::new();

    for l in lines {
        for c in l.chars() {
            grid.push(c as u8 - '0' as u8);
        }
    }

    let floor = Floor {
        grid,
        width,
        height,
    };

    // Part 1
    let mut part_1_result: usize = 0;
    let mut basin_sizes: Vec<usize> = Vec::new();
    for x in 0..width {
        for y in 0..height {
            if floor.is_lowest(x, y) {
                basin_sizes.push(floor.basin_size(x, y));
                part_1_result += floor.get(x, y) as usize + 1;
            }
        }
    }
    println!("Part 1: {}", part_1_result);

    // Part 2
    basin_sizes.sort_by(|a, b| b.cmp(a));
    let part_2_result = basin_sizes.iter().take(3).product::<usize>();
    println!("Part 2: {}", part_2_result);
}
