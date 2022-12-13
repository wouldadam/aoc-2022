/// Each input line represents items in a rucksack. Each letter is a different kind of item.
/// Rucksacks are grouped into three and only have 1 common item between them.
/// Find the common item for each group.
/// Assign it a priority where: a to z = 1 to 26, A to Z = 27 to 52.
/// Find the sum of the priorities for all rucksacks.
use std::collections::HashSet;

fn main() {
    let input = include_str!("../../assets/day03.txt");

    // Find the summed priority of badge item
    let sum = input
        .lines()
        .map(|line| line.chars().collect::<HashSet<char>>())
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|g| (&(&g[0] & &g[1]) & &g[2]).iter().last().unwrap().to_owned())
        .map(|badge_item| {
            if badge_item.is_lowercase() {
                badge_item as u32 - 'a' as u32 + 1
            } else {
                badge_item as u32 - 'A' as u32 + 27
            }
        })
        .sum::<u32>();

    println!("Sum of priorities: {}", sum);
}
