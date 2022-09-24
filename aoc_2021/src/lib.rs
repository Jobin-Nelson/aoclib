use std::str::FromStr;

pub fn read_one_per_line<T>(path: &str) -> Result<Vec<T>, std::io::Error>
where
    T: FromStr,
{
    Ok(std::fs::read_to_string(path)?
        .lines()
        .filter_map(|line| line.parse::<T>().ok())
        .collect())
}

pub mod day1;
pub mod day2;
pub mod day3;
