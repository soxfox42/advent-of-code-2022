use std::collections::HashSet;

#[derive(Copy, Clone, Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}
impl Move {
    fn offset(&self) -> (i64, i64) {
        match self {
            Self::Up => (0, 1),
            Self::Down => (0, -1),
            Self::Left => (1, 0),
            Self::Right => (-1, 0),
        }
    }
}
impl From<char> for Move {
    fn from(ch: char) -> Self {
        match ch {
            'U' => Self::Up,
            'D' => Self::Down,
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!(),
        }
    }
}

fn read_instructions(input: &str) -> Vec<Move> {
    let mut moves = Vec::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let dir = Move::from(parts.next().unwrap().chars().next().unwrap());
        let count: usize = parts.next().unwrap().parse().unwrap();

        for _ in 0..count {
            moves.push(dir);
        }
    }

    moves
}

pub fn run_a(input: &str) {
    let instructions = read_instructions(input);
    let mut head = (0, 0);
    let mut tail = (0, 0);

    let mut visited = HashSet::new();
    visited.insert(tail);

    for i in instructions {
        let offset = i.offset();
        head.0 += offset.0;
        head.1 += offset.1;
        if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
            tail.0 += (head.0 - tail.0).signum();
            tail.1 += (head.1 - tail.1).signum();
        }
        visited.insert(tail);
    }

    println!("Rope tail visted {} positions.", visited.len());
}

pub fn run_b(input: &str) {
    let instructions = read_instructions(input);
    let mut knots = vec![(0, 0); 10];

    let mut visited = HashSet::new();
    visited.insert(knots[9]);

    for i in instructions {
        let offset = i.offset();
        knots[0].0 += offset.0;
        knots[0].1 += offset.1;

        for i in 0..9 {
            if (knots[i].0 - knots[i + 1].0).abs() > 1 || (knots[i].1 - knots[i + 1].1).abs() > 1 {
                knots[i + 1].0 += (knots[i].0 - knots[i + 1].0).signum();
                knots[i + 1].1 += (knots[i].1 - knots[i + 1].1).signum();
            }
        }

        visited.insert(knots[9]);
    }

    println!("Rope tail visted {} positions.", visited.len());
}
