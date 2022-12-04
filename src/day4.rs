use std::ops::RangeInclusive;

fn subrange(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    (a.start() >= b.start() && a.end() <= b.end()) || (b.start() >= a.start() && b.end() <= a.end())
}

fn overlapping(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    a.start() <= b.end() && a.end() >= b.start()
}

fn parse_ranges(line: &str) -> Option<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    let mut parts = line.split(&['-', ',']);
    let a_start = parts.next()?.parse().ok()?;
    let a_end = parts.next()?.parse().ok()?;
    let b_start = parts.next()?.parse().ok()?;
    let b_end = parts.next()?.parse().ok()?;
    Some((a_start..=a_end, b_start..=b_end))
}

pub fn run_a(input: &str) {
    println!(
        "Completely overlapping assignments: {}",
        input
            .lines()
            .filter_map(parse_ranges)
            .filter(|(range_a, range_b)| subrange(range_a, range_b))
            .count()
    );
}

pub fn run_b(input: &str) {
    println!(
        "Overlapping assignments: {}",
        input
            .lines()
            .filter_map(parse_ranges)
            .filter(|(range_a, range_b)| overlapping(range_a, range_b))
            .count()
    );
}
