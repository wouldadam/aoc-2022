/// Takes a text file with numeric values representing calories of foods carried by elves.
/// An empty line indicates that the calories are carried by a new elf.
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

    // Sort the calories
    calories_per_elf.sort();

    // Sum the largest 3
    let sum: u32 = calories_per_elf.iter().rev().take(3).sum();

    println!("Sum of largest 3 calories: {}", sum);
}
