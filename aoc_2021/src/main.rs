fn main() {
    let day = std::env::args().skip(1).next();

    match day {
        Some(d) => run_day(d),
        None => eprintln!("Pass in a day to run the corresponding problem"),
    }
}

fn run_day(day: String) {
    let day = day.parse::<u8>().expect("Received invalid day");

    match day {
        1 => aoc_2021::day1::run(),
        2 => aoc_2021::day2::run(),
        3 => aoc_2021::day3::run(),
        _ => eprintln!("Received invalid date range, (1 <= day <= 25)"),
    }
}
