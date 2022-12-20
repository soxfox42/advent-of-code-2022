#[derive(Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
struct ShapeFromIntError;

impl TryFrom<u32> for Shape {
    type Error = ShapeFromIntError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Shape::Rock),
            1 => Ok(Shape::Paper),
            2 => Ok(Shape::Scissors),
            _ => Err(ShapeFromIntError),
        }
    }
}

enum Outcome {
    Win,
    Loss,
    Draw,
}

fn decode_line_a(line: &str) -> (Shape, Shape) {
    let opponent = match line.chars().next().unwrap() {
        'A' => Shape::Rock,
        'B' => Shape::Paper,
        'C' => Shape::Scissors,
        other => panic!("Unrecognised RPS character {other:?}"),
    };
    let you = match line.chars().nth(2).unwrap() {
        'X' => Shape::Rock,
        'Y' => Shape::Paper,
        'Z' => Shape::Scissors,
        other => panic!("Unrecognised RPS character {other:?}"),
    };

    (opponent, you)
}

fn decode_line_b(line: &str) -> (Shape, Outcome) {
    let opponent = match line.chars().next().unwrap() {
        'A' => Shape::Rock,
        'B' => Shape::Paper,
        'C' => Shape::Scissors,
        other => panic!("Unrecognised RPS character {other:?}"),
    };
    let outcome = match line.chars().nth(2).unwrap() {
        'X' => Outcome::Loss,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        other => panic!("Unrecognised RPS outcome character {other:?}"),
    };

    (opponent, outcome)
}

fn finish_round(round: (Shape, Outcome)) -> (Shape, Shape) {
    match round {
        (shape, Outcome::Win) => (shape, ((shape as u32 + 1) % 3).try_into().unwrap()),
        (shape, Outcome::Loss) => (shape, ((shape as u32 + 2) % 3).try_into().unwrap()),
        (shape, Outcome::Draw) => (shape, shape),
    }
}

fn score(round: (Shape, Shape)) -> u32 {
    let opponent = round.0 as u32;
    let you = round.1 as u32;

    if (opponent + 1) % 3 == you {
        you + 7
    } else if (you + 1) % 3 == opponent {
        you + 1
    } else {
        you + 4
    }
}

pub fn run_a(input: &str) {
    println!(
        "Total Score: {}",
        input.lines().map(decode_line_a).map(score).sum::<u32>()
    );
}
pub fn run_b(input: &str) {
    println!(
        "Total Score: {}",
        input
            .lines()
            .map(decode_line_b)
            .map(finish_round)
            .map(score)
            .sum::<u32>()
    );
}
