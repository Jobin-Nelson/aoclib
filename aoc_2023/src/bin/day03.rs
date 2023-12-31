#[derive(Debug)]
struct Part {
    start: (usize, usize),
    length: usize,
    number: usize,
}

#[derive(Debug)]
struct Parts<'a> {
    graph: &'a Vec<Vec<u8>>,
    index: (usize, usize),
    width: usize,
    height: usize,

}

impl<'a> Parts<'a> {
    fn new(graph: &'a Vec<Vec<u8>>) -> Parts<'a> {
        let height = graph.len();
        let width = match graph.get(0) {
            Some(l) => l.len(),
            None => 0
        };
        Self {
            graph,
            width,
            height,
            index: (0,0),
        }
    }
}

impl Iterator for Parts<'_> {
    type Item = Part;

    fn next(&mut self) -> Option<Self::Item> {
        for i in self.index.0..self.height {
            for j in self.index.1..self.width {
                let d = self.graph[i][j];
                if d >= b'0' && d <= b'9' {
                    let start = (i, j);
                    let iter = self.graph[i][j..].iter();
                    let (number, length) = iter
                        .take_while(|d| d.is_ascii_digit())
                        .fold((0, 0), |acc, x| {
                            (acc.0 * 10 + (x - b'0') as usize, acc.1 + 1)
                        });
                    self.index = (i, j + length);
                    return Some(Part{
                        start,
                        number,
                        length,
                    });
                }
            }
            self.index.1 = 0;
        }
        None
    }
} 


fn main() {
    let input = std::fs::read_to_string("data/day03.txt").unwrap();

    println!("Part 1: {}", part_1(&input));
}

fn part_1(graph: &str) -> usize {
    let graph = graph.lines().map(|l| l.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();
    let parts = Parts::new(&graph);
    parts.filter_map(|p| {
        if is_valid(&graph, &p) {
            return Some(p.number)
        }
        None
    }).sum()
}

fn is_valid(graph: &Vec<Vec<u8>>, part: &Part) -> bool {
    let (x, y) = part.start;
    let mut dirs: Vec<(Option<usize>, Option<usize>)> = Vec::new();

    for i in 0..part.length {
        dirs.push((x.checked_sub(1), (y+i).checked_sub(1)));
        dirs.push((x.checked_sub(1), Some(y+i)));
        dirs.push((x.checked_sub(1), (y+i).checked_add(1)));
        dirs.push((Some(x), (y+i).checked_sub(1)));
        dirs.push((Some(x), (y+i).checked_add(1)));
        dirs.push((x.checked_add(1), (y+i).checked_sub(1)));
        dirs.push((x.checked_add(1), Some(y+i)));
        dirs.push((x.checked_add(1), (y+i).checked_add(1)));
    }

    dirs.iter().any(|d| {
        let (x, y) = d;
        if x.is_none() || y.is_none() { return false }
        if let Some(v) = graph.get(x.unwrap()).and_then(|v| v.get(y.unwrap())){
            return !v.is_ascii_digit() && v != &b'.'
        }
        return false;
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
    #[test]
    fn test_1() {
        let expected = 4361;
        let actual = part_1(&INPUT);
        assert_eq!(expected, actual);
    }
}
