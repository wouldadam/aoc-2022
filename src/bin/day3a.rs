/// Each input line represents items in a rucksack. Each letter is a different kind of item.
/// Each rucksack has 2 compartments, the line is split in half for each compartment.
/// Find the common item between the two compartments for each rucksack.
/// Assign it a priority where: a to z = 1 to 26, A to Z = 27 to 52.
/// Find the sum of the priorities for all rucksacks.
use std::collections::HashSet;
fn main() {
    let input = include_str!("../../assets/day3a.txt");

    // Find the priority score for each rucksack
    let priority_per_rucksack = input.lines().map(|rucksack| {
        let (pocket1, pocket2) = rucksack.split_at(rucksack.len() / 2);
        let set: HashSet<char> = pocket1.chars().collect();
        let common = pocket2.chars().find(|c| set.contains(c)).unwrap();

        if common.is_lowercase() {
            common as i32 - 'a' as i32 + 1
        } else {
            common as i32 - 'A' as i32 + 27
        }
    });

    // Sum the priorities
    let priority_sum: i32 = priority_per_rucksack.sum();

    println!("Sum of priorities: {}", priority_sum);
}
