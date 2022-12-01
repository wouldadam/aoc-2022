/// Takes a text file with numeric values representing calories of foods carried by elves.
/// An empty line indicates that the calories are carried by a new elf.
/// Prints the largest sum of the calories carried by a single elf.
fn main() {
    let input = include_str!("../../assets/day1a.txt");

    // Parse the file into foods for each elf
    let foods_per_elf = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|cal| cal.parse::<u32>().unwrap()));

    // Sum the calories for each elf
    let calories_per_elf = foods_per_elf.map(|foods| foods.sum::<u32>());

    // Find the largest number
    let most = calories_per_elf.max().unwrap();

    println!("Most calories: {}", most);
}
