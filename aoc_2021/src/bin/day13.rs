use std::collections::HashSet;

#[derive(Debug)]
struct Paper {
    grid: HashSet<(i32, i32)>,
    width: usize,
    height: usize,
}

impl Paper {
    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid.contains(&(x as i32, y as i32)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn fold_y(&mut self, ypos: usize) {
        let mut new_grid = HashSet::new();
        let new_height = if ypos <= self.height / 2 {
            self.height - ypos - 1
        } else {
            ypos
        };

        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid.contains(&(x as i32, y as i32)) {
                    let new_y = ypos as i32 - i32::abs(y as i32 - ypos as i32);
                    new_grid.insert((x as i32, new_y));
                }
            }
        }

        self.grid = new_grid;
        self.height = new_height;
    }

    fn fold_x(&mut self, xpos: usize) {
        let mut new_grid = HashSet::new();
        let new_width = if xpos <= self.width {
            self.width - xpos - 1
        } else {
            xpos
        };

        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid.contains(&(x as i32, y as i32)) {
                    let new_x = xpos as i32 - i32::abs(x as i32 - xpos as i32);
                    new_grid.insert((new_x, y as i32));
                }
            }
        }

        self.grid = new_grid;
        self.width = new_width;
    }
}
fn main() {
    let input = std::fs::read_to_string("data/day13.txt").unwrap();
    let lines: Vec<String> = input.lines().map(|l| l.to_string()).collect();

    let mut grid = HashSet::new();
    let mut width = 0;
    let mut height = 0;

    let mut idx = 0;

    while !lines[idx].is_empty() {
        if let Some((x, y)) = lines[idx].split_once(',') {
            let x: i32 = x.parse().unwrap();
            let y: i32 = y.parse().unwrap();

            if x as usize > width {
                width = x as usize;
            }
            if y as usize > height {
                height = y as usize;
            }
            grid.insert((x, y));
        }
        idx += 1;
    }
    idx += 1; // skip the blank line

    let mut paper = Paper {
        grid,
        width: width + 1,
        height: height + 1,
    };

    let mut part_1_done = false;

    while idx < lines.len() {
        let mut split = lines[idx].split(' ');
        split.next();
        split.next();
        if let Some((axis, strline)) = split.next().unwrap().split_once('=') {
            let num: usize = strline.parse().unwrap();
            if axis == "x" {
                paper.fold_x(num);
            } else {
                paper.fold_y(num);
            }
        }
        if !part_1_done {
            println!("Part 1: {}", paper.grid.len());
            part_1_done = true;
        }
        idx += 1;
    }

    println!("Part 2: ");
    paper.print();
}
