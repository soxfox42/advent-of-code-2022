fn group_sums(input: String) -> Vec<i32> {
    return input
        .trim()
        .split("\n\n")
        .map(|group| group.split("\n").map(|cal| cal.parse::<i32>().unwrap()))
        .map(|group| group.sum::<i32>())
        .collect();
}

pub fn run_a(input: String) {
    println!("Max: {}", group_sums(input).iter().max().unwrap());
}

pub fn run_b(_input: String) {}
