fn read_grid(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .collect()
}

fn score_one(trees: &[Vec<u32>], x: usize, y: usize) -> u32 {
    let here = trees[y][x];

    let mut score = 1;

    let mut dir_score = 0;
    for x in (0..x).rev() {
        dir_score += 1;
        if trees[y][x] >= here {
            break;
        }
    }
    score *= dir_score;
    dir_score = 0;
    for x in (x + 1)..trees[y].len() {
        dir_score += 1;
        if trees[y][x] >= here {
            break;
        }
    }
    score *= dir_score;
    dir_score = 0;
    for y in (0..y).rev() {
        dir_score += 1;
        if trees[y][x] >= here {
            break;
        }
    }
    score *= dir_score;
    dir_score = 0;
    for y in (y + 1)..trees.len() {
        dir_score += 1;
        if trees[y][x] >= here {
            break;
        }
    }
    score *= dir_score;

    score
}

fn score(trees: &[Vec<u32>]) -> Vec<Vec<u32>> {
    let mut scores = Vec::new();

    for y in 0..trees.len() {
        scores.push(Vec::new());
        for x in 0..trees[y].len() {
            scores[y].push(score_one(trees, x, y));
        }
    }

    scores
}

pub fn run_a(input: &str) {
    let trees = read_grid(input);
    let mut visibility = vec![vec![false; trees[0].len()]; trees.len()];

    for y in 0..trees.len() {
        let mut max = None;
        for x in 0..trees[y].len() {
            if max.map(|max| trees[y][x] > max).unwrap_or(true) {
                visibility[y][x] = true;
                max = Some(trees[y][x]);
            }
        }

        max = None;
        for x in (0..trees[y].len()).rev() {
            if max.map(|max| trees[y][x] > max).unwrap_or(true) {
                visibility[y][x] = true;
                max = Some(trees[y][x]);
            }
        }
    }

    for x in 0..trees[0].len() {
        let mut max = None;
        for y in 0..trees.len() {
            if max.map(|max| trees[y][x] > max).unwrap_or(true) {
                visibility[y][x] = true;
                max = Some(trees[y][x]);
            }
        }

        max = None;
        for y in (0..trees.len()).rev() {
            if max.map(|max| trees[y][x] > max).unwrap_or(true) {
                visibility[y][x] = true;
                max = Some(trees[y][x]);
            }
        }
    }

    println!(
        "I can see {} trees.",
        visibility
            .into_iter()
            .map(|row| row.into_iter().map(|x| x as u32).sum::<u32>())
            .sum::<u32>()
    );
}

pub fn run_b(input: &str) {
    let trees = read_grid(input);
    let scores = score(&trees);
    println!("The highest score is {}.", scores.iter().map(|row| row.iter().max().unwrap()).max().unwrap());
}
