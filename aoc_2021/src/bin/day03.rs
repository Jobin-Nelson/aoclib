pub fn main() {
    let size = 32;
    let lines: Vec<Vec<u32>> = std::fs::read_to_string("data/day03.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| char::to_digit(c, 2).unwrap())
                .collect()
        })
        .collect();

    let digits = lines[0].len();
    let half = lines.len() as u32 / 2;
    let mut sums = vec![0; digits];

    lines.iter().for_each(|line| {
        for i in 0..digits {
            sums[i] += line[i]
        }
    });

    let gamma: u32 = sums
        .into_iter()
        .map(|d| if d > half { 1 } else { 0 })
        .fold(0, |acc, digit| (acc << 1) + digit);
    let epsilon: u32 = !gamma << size - digits >> size - digits;

    let filter_vals = |f: fn(u32, u32) -> bool| {
        let mut values = lines.clone();
        for idx in 0..digits {
            let line_values: Vec<_> = values.iter().map(|digits| digits[idx]).collect();

            let oxygen_half = (line_values.len() as u32 + 1) / 2;
            let oxygen_sum = line_values.iter().sum::<u32>();
            let most_common = if oxygen_sum < oxygen_half { 0 } else { 1 };

            values = values
                .into_iter()
                .filter(|value| f(value[idx], most_common))
                .collect();

            if values.len() == 1 {
                break;
            }
        }
        values
    };

    let oxygen_values = filter_vals(|digit, most_common| digit == most_common);
    let co2_values = filter_vals(|digit, most_common| digit != most_common);

    let oxygen_value = oxygen_values[0]
        .iter()
        .fold(0, |res, digit| (res << 1) + digit);

    let co2_value = co2_values[0]
        .iter()
        .fold(0, |res, digit| (res << 1) + digit);

    println!("Part 1: {}", gamma * epsilon);
    println!("Part 2: {}", oxygen_value * co2_value);
}
