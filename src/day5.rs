use std::num::ParseIntError;

type Stacks = Vec<Vec<char>>;
#[derive(Debug)]
struct Move {
    count: u64,
    src: usize,
    dest: usize,
}

fn parse_stacks(input: &str) -> Stacks {
    let count = (input.lines().next().unwrap().len() + 3) / 4;
    let mut stacks = vec![Vec::new(); count];

    for line in input.lines() {
        for (i, crt) in line.chars().skip(1).step_by(4).enumerate() {
            if crt.is_alphabetic() {
                stacks[i].push(crt);
            }
        }
    }

    for stack in &mut stacks {
        stack.reverse();
    }

    stacks
}

fn parse_moves(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let count = parts[1].parse()?;
            let src = parts[3].parse::<usize>()? - 1;
            let dest = parts[5].parse::<usize>()? - 1;
            Ok(Move { count, src, dest })
        })
        .collect::<Result<Vec<Move>, ParseIntError>>()
        .unwrap()
}

fn parse_input(input: &str) -> (Stacks, Vec<Move>) {
    let (stacks, moves) = input.split_once("\n\n").unwrap();
    (parse_stacks(stacks), parse_moves(moves))
}

pub fn run_a(input: &str) {
    let (mut stacks, moves) = parse_input(input);

    for Move { count, src, dest } in moves {
        for _ in 0..count {
            let crt = stacks[src].pop().unwrap();
            stacks[dest].push(crt);
        }
    }

    println!(
        "Stack tops: {}",
        stacks.iter().map(|stack| stack.last().unwrap()).collect::<String>()
    );
}

pub fn run_b(_input: &str) {}
