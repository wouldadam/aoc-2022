/// Each input line represents items in a rucksack. Each letter is a different kind of item.
/// Find the common item between the two compartments for each rucksack.
/// Assign it a priority where: a to z = 1 to 26, A to Z = 27 to 52.
/// Find the sum of the priorities for all rucksacks.
/// Part A:
/// Each rucksack has 2 compartments, the line is split in half for each compartment.
/// Part B:
/// Rucksacks are grouped into three and only have 1 common item between them.
use std::collections::HashSet;

fn main() {
    let input = include_str!("../../assets/day03.txt");

    // Find the priority score for each rucksack
    let sum_common_priority = input
        .lines()
        .map(|rucksack| {
            let (pocket1, pocket2) = rucksack.split_at(rucksack.len() / 2);
            let set: HashSet<char> = pocket1.chars().collect();
            let common = pocket2.chars().find(|c| set.contains(c)).unwrap();

            if common.is_lowercase() {
                common as i32 - 'a' as i32 + 1
            } else {
                common as i32 - 'A' as i32 + 27
            }
        })
        .sum::<i32>();

    println!("Sum of common item priorities: {}", sum_common_priority);

    // Find the summed priority of badge item
    let sum_badge_priority = input
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

    println!("Sum of badge priorities: {}", sum_badge_priority);
}
