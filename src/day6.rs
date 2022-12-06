fn all_unique(seq: &[char]) -> bool {
    for ch in seq {
        if seq.iter().filter(|&x| x == ch).count() > 1 {
            return false;
        }
    }
    true
}

pub fn run_a(input: &str) {
    let chars: Vec<char> = input.chars().collect();
    println!("Marker found at position {}.", chars.windows(4).position(all_unique).unwrap() + 4);
}

pub fn run_b(_input: &str) {}
