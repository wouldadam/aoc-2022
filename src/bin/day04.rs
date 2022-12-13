/// Input data describes pairs of ranges of job assignment ids.
/// Some pairs are fully contained by the other.
/// Part A:
/// Count the number of pair when one assignment contains the other.
/// Part B:
/// Count the number of pairs that overlap at all.
use regex::Regex;

fn main() {
    let input = include_str!("../../assets/day04.txt");

    // Parse into individual lines
    let pairs = input.lines();

    // Parse each pair into two assignments, count the ones that overlap
    let reg = Regex::new(r"^(\d*)\-(\d*),(\d*)\-(\d*)$").unwrap();

    let mut contain_count = 0;
    let mut overlap_count = 0;
    for pair in pairs {
        let captures = reg.captures(pair).unwrap();

        let e1 = (
            captures[1].parse::<u32>().unwrap(),
            captures[2].parse::<u32>().unwrap(),
        );
        let e2 = (
            captures[3].parse::<u32>().unwrap(),
            captures[4].parse::<u32>().unwrap(),
        );

        if (e1.0 >= e2.0 && e1.1 <= e2.1) || (e2.0 >= e1.0 && e2.1 <= e1.1) {
            contain_count += 1;
        }

        if e1.0 <= e2.1 && e2.0 <= e1.1 {
            overlap_count += 1;
        }
    }

    println!("Contain count: {}", contain_count);
    println!("Overlap count: {}", overlap_count);
}
