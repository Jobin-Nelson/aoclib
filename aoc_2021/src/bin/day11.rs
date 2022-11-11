const DIRS: [(i32, i32); 8] = [
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

#[derive(Debug)]
struct OctupusGrid {
    grid: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl OctupusGrid {
    fn step(&mut self) -> usize {
        let mut flashers: Vec<(i32, i32)> = Vec::new();
        let mut flashed = 0;

        for r in 0..self.height {
            for c in 0..self.width {
                self.grid[r][c] += 1;
                if self.grid[r][c] > 9 {
                    flashers.push((r as i32, c as i32));
                }
            }
        }

        while let Some((x, y)) = flashers.pop() {
            flashed += 1;

            for d in DIRS {
                let (dx, dy) = (x + d.0, y + d.1);

                if let Some(val) = self.grid.get(dx as usize).and_then(|v| v.get(dy as usize)) {
                    if val <= &9 {
                        self.grid[dx as usize][dy as usize] += 1;
                        if self.grid[dx as usize][dy as usize] > 9 {
                            flashers.push((dx, dy));
                        }
                    }
                }
            }
        }

        for r in 0..self.height {
            for c in 0..self.width {
                if self.grid[r][c] > 9 {
                    self.grid[r][c] = 0;
                }
            }
        }

        flashed
    }
}

fn main() {
    let input = std::fs::read_to_string("data/day11.txt").unwrap();
    let mut grid: Vec<Vec<u8>> = Vec::new();

    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as u8);
        }
        grid.push(row);
    }
    let width = grid[0].len();
    let height = grid.len();

    let mut octupus_grid = OctupusGrid {
        grid,
        width,
        height,
    };

    let mut part_1_done = false;
    let mut part_2_done = false;
    let mut part_1_result = 0;
    let mut part_2_result = 0;
    let all_flash = octupus_grid.width * octupus_grid.height;
    let mut steps = 0;

    while !part_1_done || !part_2_done {
        let flash_count = octupus_grid.step();
        steps += 1;

        if steps <= 100 {
            part_1_result += flash_count;
        } else {
            part_1_done = true;
        }

        if flash_count == all_flash {
            part_2_result = steps;
            part_2_done = true;
        }
    }

    println!("Part 1: {}", part_1_result);
    println!("Part 2: {}", part_2_result);
}
