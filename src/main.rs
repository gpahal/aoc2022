mod utils;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

const MAX_IMPLEMENTED_DAY: u8 = 11;
const MAX_IMPLEMENTED_PART: u8 = 2;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let program_name = &args[0];
    let help = format!("Usage:\n  Cargo run: cargo run -- [day=1..{MAX_IMPLEMENTED_DAY}] [part=1..2]\n  Executable: {program_name} [day=1..{MAX_IMPLEMENTED_DAY}] [part=1..2]");
    if let Err(message) = run(&args[1..]) {
        println!("[error] {message}\n\n{help}");
    }
}

fn run(args: &[String]) -> Result<(), String> {
    let args_len = args.len();
    if args_len < 1 {
        return Err(format!("Please specify which challenge you want to run: day (1 to {MAX_IMPLEMENTED_DAY}) and part (1 or 2)"));
    } else if args_len > 2 {
        return Err("Too many arguments provided".to_string());
    }

    let day = parse_day(&args[0])?;
    if args_len < 2 {
        if day == MAX_IMPLEMENTED_DAY && MAX_IMPLEMENTED_PART == 1 {
            run_challenge(day, 1)
        } else {
            Err(format!(
                "Please specify which challenge part you want to run for day {day}: 1 or 2"
            ))
        }
    } else {
        let part = parse_part(day, &args[1])?;
        run_challenge(day, part)
    }
}

fn run_challenge(day: u8, part: u8) -> Result<(), String> {
    match (day, part) {
        (1, 1) => day01::part1(),
        (1, 2) => day01::part2(),
        (2, 1) => day02::part1(),
        (2, 2) => day02::part2(),
        (3, 1) => day03::part1(),
        (3, 2) => day03::part2(),
        (4, 1) => day04::part1(),
        (4, 2) => day04::part2(),
        (5, 1) => day05::part1(),
        (5, 2) => day05::part2(),
        (6, 1) => day06::part1(),
        (6, 2) => day06::part2(),
        (7, 1) => day07::part1(),
        (7, 2) => day07::part2(),
        (8, 1) => day08::part1(),
        (8, 2) => day08::part2(),
        (9, 1) => day09::part1(),
        (9, 2) => day09::part2(),
        (10, 1) => day10::part1(),
        (10, 2) => day10::part2(),
        (11, 1) => day11::part1(),
        (11, 2) => day11::part2(),
        _ => {}
    }
    Ok(())
}

fn parse_day(day_str: &str) -> Result<u8, String> {
    let day = day_str
        .parse::<u8>()
        .map_err(|_| format!("First argument needs to be the day of the challenge: {day_str}"))?;
    if day < 1 || day > 25 {
        Err(format!("First argument needs to be the day of the challenge and should be in the range: 1 to {MAX_IMPLEMENTED_DAY}, got {day}"))
    } else if day > MAX_IMPLEMENTED_DAY {
        Err(format!(
            "Day {day} challenges have not been implemented yet"
        ))
    } else {
        Ok(day)
    }
}

fn parse_part(day: u8, part_str: &str) -> Result<u8, String> {
    let part = part_str.parse::<u8>().map_err(|_| {
        format!("Second argument needs to be the challenge part for the day {day}: 1 or 2")
    })?;
    if part != 1 && part != 2 {
        Err(format!("Second argument needs to be the challenge part for the day {day} and should be: 1 or 2"))
    } else if day == MAX_IMPLEMENTED_DAY && part > MAX_IMPLEMENTED_PART {
        Err(format!(
            "Challenge part {part} for day {day} has not been implemented yet"
        ))
    } else {
        Ok(part)
    }
}
