enum Shape {
    Rock,
    Paper,
    Scissors,
}

fn decode_line(line: &str) -> (Shape, Shape) {
    let opponent = match line.chars().nth(0).unwrap() {
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
    println!("Total Score: {}", input.lines().map(decode_line).map(score).sum::<u32>());
}
pub fn run_b(_input: &str) {}
