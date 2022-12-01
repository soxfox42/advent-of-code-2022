mod day1;

use std::env;
use std::error::Error;
use std::fs;

const DAYS: &[(fn(&str), fn(&str))] = &[(day1::run_a, day1::run_b)];

fn run_day(i: usize) -> Result<(), Box<dyn Error>> {
    let (run_a, run_b) = DAYS[i - 1];

    let path = format!("inputs/day{}.txt", i);
    let input = fs::read_to_string(path)?;

    println!("Day {}, Part A", i);
    run_a(&input);
    println!();
    println!("Day {}, Part B", i);
    run_b(&input);
    println!();

    Ok(())
}

fn main() {
    let args = env::args();

    if args.len() == 1 {
        for i in 1..=DAYS.len() {
            run_day(i).unwrap();
        }
    }
}
