fn main() {
    let input = std::fs::read_to_string("data/day17.txt").unwrap();
    let (x_part, y_part) = input.trim_end().split_once(", ").unwrap();
    let (_, x_range) = x_part.split_once('=').unwrap();
    let (_, y_range) = y_part.split_once('=').unwrap();
    let (minx, maxx) = x_range.split_once("..").unwrap();
    let (miny, maxy) = y_range.split_once("..").unwrap();
    let minx: i64 = minx.parse().unwrap();
    let maxx: i64 = maxx.parse().unwrap();
    let miny: i64 = miny.parse().unwrap();
    let maxy: i64 = maxy.parse().unwrap();

    println!("Part 1: {}", part_1(miny));
    println!("Part 2: {}", part_2(minx, maxx, miny, maxy));
}

fn part_1(miny: i64) -> i64 {
    miny * (miny + 1) / 2
}

fn part_2(minx: i64, maxx: i64, miny: i64, maxy: i64) -> usize {
    let mut count = 0;

    for y in miny..=1 - miny {
        for x in 0..=maxx {
            let mut xpos = 0;
            let mut ypos = 0;

            for t in 0..1000 {
                ypos += y - t;
                if x - t > 0 {
                    xpos += x - t;
                }

                if miny <= ypos && ypos <= maxy && minx <= xpos && xpos <= maxx {
                    count += 1;
                    break;
                }
            }
        }
    }

    count
}
