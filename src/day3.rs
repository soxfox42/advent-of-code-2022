use std::collections::HashSet;

fn priority(item: char) -> u32 {
    if ('a'..='z').contains(&item) {
        item as u32 - 'a' as u32 + 1
    } else {
        item as u32 - 'A' as u32 + 27
    }
}

fn find_common(line: &str) -> char {
    let mid = line.len() / 2;
    let (left, right) = line.split_at(mid);
    let left_chars: HashSet<char> = left.chars().collect();
    let right_chars: HashSet<char> = right.chars().collect();
    *left_chars.intersection(&right_chars).next().unwrap()
}

pub fn run_a(input: &str) {
    println!(
        "Sum of priorities: {}",
        input.lines().map(find_common).map(priority).sum::<u32>()
    );
}

pub fn run_b(_input: &str) {}
