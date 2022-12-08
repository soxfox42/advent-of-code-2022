mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

use std::env;
use std::error::Error;
use std::fs;

const DAYS: &[(fn(&str), fn(&str))] = &[
    (day1::run_a, day1::run_b),
    (day2::run_a, day2::run_b),
    (day3::run_a, day3::run_b),
    (day4::run_a, day4::run_b),
    (day5::run_a, day5::run_b),
    (day6::run_a, day6::run_b),
    (day7::run_a, day7::run_b),
    (day8::run_a, day8::run_b),
];

fn run_day(i: usize) -> Result<(), Box<dyn Error>> {
    let (run_a, run_b) = DAYS[i - 1];

    let path = format!("inputs/day{}.txt", i);
    let input = fs::read_to_string(path)?;

    println!("Day {}, Part A", i);
    run_a(input.trim());
    println!();
    println!("Day {}, Part B", i);
    run_b(input.trim());
    println!();

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();

    if args.len() == 1 {
        for i in 1..=DAYS.len() {
            run_day(i)?;
        }
    } else {
        let i = args.nth(1).unwrap().parse()?;
        run_day(i)?;
    }

    Ok(())
}
