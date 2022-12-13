/// Takes a text file with numeric values representing calories of foods carried by elves.
/// An empty line indicates that the calories are carried by a new elf.
/// Part A:
/// Print the largest sum of the calories carried by a single elf.
/// Part B:
/// Prints the sum of the calories carried by the 3 largest calorie carrying single elves.

fn main() {
    let input = include_str!("../../assets/day01.txt");

    // Parse the file into foods for each elf
    let foods_per_elf = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|cal| cal.parse::<u32>().unwrap()));

    // Sum the calories for each elf
    let mut calories_per_elf = foods_per_elf
        .map(|foods| foods.sum::<u32>())
        .collect::<Vec<_>>();

    // Find the largest number
    let most = calories_per_elf.iter().max().unwrap();
    println!("Most calories: {}", most);

    // Sum the largest 3
    calories_per_elf.sort();
    let sum: u32 = calories_per_elf.iter().rev().take(3).sum();
    println!("Sum of largest 3 calories: {}", sum);
}
