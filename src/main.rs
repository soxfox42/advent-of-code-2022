mod day1;

use std::env;
use std::error::Error;
use std::fs;

const DAYS: &[(fn(String), fn(String))] = &[(day1::run_a, day1::run_b)];

fn run_day(i: usize) -> Result<(), Box<dyn Error>> {
    let (run_a, run_b) = DAYS[i - 1];

    let path_a = format!("inputs/day{}a.txt", i);
    let path_b = format!("inputs/day{}b.txt", i);
    let input_a = fs::read_to_string(path_a)?;
    let input_b = fs::read_to_string(path_b)?;

    println!("Day {}, Part A", i);
    run_a(input_a);
    println!();
    println!("Day {}, Part B", i);
    run_b(input_b);
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
