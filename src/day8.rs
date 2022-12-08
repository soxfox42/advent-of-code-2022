fn read_grid(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .collect()
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

pub fn run_b(_input: &str) {}
