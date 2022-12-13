use std::collections::HashSet;

/// Detect the first occurrence of 4 unique chars in the input.
/// Part A:
/// Print char count up to the end of the 4 unique chars.
/// Part B:
/// Print char count up to the end of the 14 unique chars.

fn detect_unique(input: &str, unique_count: usize) {
    let start_idx = input
        .chars()
        .collect::<Vec<_>>()
        .windows(unique_count)
        .position(|window| HashSet::<&char>::from_iter(window.iter()).len() == unique_count);

    println!("End idx {}", start_idx.unwrap() + unique_count);
}

fn main() {
    let input = include_str!("../../assets/day06.txt");

    detect_unique(input, 4);
    detect_unique(input, 14);
}
