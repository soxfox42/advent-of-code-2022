pub fn run_a(input: String) {
    let groups = input
        .trim()
        .split("\n\n")
        .map(|group| group.split("\n").map(|cal| cal.parse::<i32>().unwrap()));
    let sums = groups.map(|group| group.sum::<i32>());
    println!("Max: {}", sums.max().unwrap());
}

pub fn run_b(_input: String) {}
