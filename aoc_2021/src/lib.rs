use std::str::FromStr;

pub mod day01;

pub fn read_one_per_line<T>(path: &str) -> Result<Vec<T>, std::io::Error> 
where
    T: FromStr
{
    Ok(std::fs::read_to_string(path)?
       .split("\n")
       .filter_map(|line| line.parse::<T>().ok())
       .collect())
}
