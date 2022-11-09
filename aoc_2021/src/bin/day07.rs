fn main() {
    let input = std::fs::read_to_string("data/day07.txt").unwrap();
    let mut submarines: Vec<i32> = input
        .trim_end()
        .split(",")
        .map(|i| i.parse().unwrap())
        .collect();

    submarines.sort();

    // Part 1
    {
        let middle = submarines.len() / 2;
        let median = submarines[middle];
        let result = submarines
            .iter()
            .fold(0, |acc, i| i32::abs(i - median) + acc);
        println!("Part 1: {}", result);
    }

    // Part 2
    {
        let submarines: Vec<f32> = submarines.iter().map(|i| *i as f32).collect();

        let mean = submarines.iter().sum::<f32>() / submarines.len() as f32;
        let floor_result = submarines.iter().fold(0, |acc, i| {
            let n = f32::abs(i - mean.floor());
            (n * (n + 1.0) / 2.0) as i32 + acc
        });

        let ceil_result = submarines.iter().fold(0, |acc, i| {
            let n = f32::abs(i - mean.ceil());
            (n * (n + 1.0) / 2.0) as i32 + acc
        });
        println!("Part 2: {}", floor_result.min(ceil_result));
    }
}
