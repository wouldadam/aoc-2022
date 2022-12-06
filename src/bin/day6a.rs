use std::collections::HashSet;

/// Detect the first occurrence of 4 unique chars in the input.
/// Print char count up to the end of the 4 unique chars.
fn main() {
    let input = include_str!("../../assets/day6a.txt");

    let start_idx = input
        .chars()
        .collect::<Vec<_>>()
        .windows(4)
        .position(|window| HashSet::<&char>::from_iter(window.iter()).len() == 4);

    println!("End idx {}", start_idx.unwrap() + 4);
}
