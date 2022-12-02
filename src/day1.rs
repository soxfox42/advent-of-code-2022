fn group_sums(input: &str) -> Vec<i32> {
    return input
        .split("\n\n")
        .map(|group| group.split("\n").map(|cal| cal.parse::<i32>().unwrap()))
        .map(|group| group.sum::<i32>())
        .collect();
}

pub fn run_a(input: &str) {
    println!("Max: {}", group_sums(input).iter().max().unwrap());
}

pub fn run_b(input: &str) {
    let mut sums = group_sums(input);
    sums.sort();
    sums.reverse();
    println!("Sum of 3 Max: {}", sums[..3].iter().sum::<i32>());
}
