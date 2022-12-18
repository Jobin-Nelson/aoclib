use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Debug)]
struct HeightMap {
    grid: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
    width: usize,
    height: usize,
}

impl FromStr for HeightMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Vec::new();
        let mut start = (0, 0);
        let mut end = (0, 0);
        for (i, line) in s.lines().enumerate() {
            let mut row: Vec<u8> = line.as_bytes().to_vec();
            if let Some(start_point) = row.iter().position(|&p| p == b'S') {
                start = (i, start_point);
                row[start_point] = b'a';
            }
            if let Some(end_point) = row.iter().position(|&p| p == b'E') {
                end = (i, end_point);
                row[end_point] = b'z';
            }
            grid.push(row);
        }

        let width = grid[0].len();
        let height = grid.len();
        Ok(Self {
            grid,
            width,
            height,
            start,
            end,
        })
    }
}

impl HeightMap {
    fn get_surrounding_points(
        &self,
        pos: (usize, usize),
        f: &dyn Fn(u8, u8) -> bool,
    ) -> Vec<(usize, usize)> {
        let ipos = (pos.0 as i32, pos.1 as i32);
        let width = self.width as i32;
        let height = self.height as i32;
        let cur_elevation = self.grid[pos.0][pos.1];

        DIRS.iter()
            .map(|d| (ipos.0 + d.0, ipos.1 + d.1))
            .filter(|pos| pos.0 >= 0 && pos.1 >= 0 && pos.0 < height && pos.1 < width)
            .map(|pos| (pos.0 as usize, pos.1 as usize))
            .filter(|pos| f(self.grid[pos.0][pos.1], cur_elevation))
            .collect()
    }

    fn bfs(&self, start_point: (usize, usize), backwards: bool) -> Option<usize> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        visited.insert(start_point);
        queue.push_back((start_point, 0));

        while let Some(loc) = queue.pop_front() {
            let valid = if backwards {
                self.get_surrounding_points(loc.0, &|new_elev: u8, cur_elev: u8| {
                    cur_elev <= new_elev + 1
                })
            } else {
                self.get_surrounding_points(loc.0, &|new_elev: u8, cur_elev: u8| {
                    new_elev <= cur_elev + 1
                })
            };

            for v in valid {
                if !visited.insert(v) {
                    continue;
                }

                if (backwards && self.grid[v.0][v.1] == b'a') || (!backwards && v == self.end) {
                    return Some(loc.1 + 1);
                }

                queue.push_back((v, loc.1 + 1));
            }
        }

        None
    }
}

fn main() {
    let input = std::fs::read_to_string("data/day12.txt").unwrap();
    let heightmap: HeightMap = input.parse().unwrap();

    println!("Part 1: {}", heightmap.bfs(heightmap.start, false).unwrap());
    println!("Part 2: {}", heightmap.bfs(heightmap.end, true).unwrap());

    // let (r, c) = heightmap.start;
    // let start_value = heightmap.grid[r][c];
    // println!(
    //     "Part 1: {}",
    //     explore(&heightmap, r as i32, c as i32, start_value, &mut visited, 0).unwrap() - 2
    // );
}

// fn explore(
//     hmap: &HeightMap,
//     row: i32,
//     col: i32,
//     prev: u8,
//     visited: &mut HashSet<(i32, i32)>,
//     count: usize,
// ) -> Option<usize> {
//     if (row as usize, col as usize) == hmap.end {
//         return Some(count);
//     }
//     if row < 0
//         || row >= hmap.height as i32
//         || col < 0
//         || col >= hmap.width as i32
//         || visited.contains(&(row, col))
//         || hmap.grid[row as usize][col as usize] > (prev + 1)
//     {
//         return None;
//     }
//
//     visited.insert((row, col));
//
//     let mut res = usize::MAX;
//     for (dx, dy) in DIRS {
//         let (r, c) = (row + dx, col + dy);
//         if let Some(val) = explore(
//             hmap,
//             r,
//             c,
//             hmap.grid[row as usize][col as usize],
//             visited,
//             count + 1,
//         ) {
//             res = res.min(val);
//         };
//     }
//
//     visited.remove(&(row, col));
//
//     if res == usize::MAX {
//         None
//     } else {
//         Some(res)
//     }
// }
