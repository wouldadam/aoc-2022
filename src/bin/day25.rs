use std::time;

use num::Integer;

/// Input is a list of fuel requirements
/// Each number is in the SNAFU format
/// SNAFU's bases are powers of 5: 125,25,5,1
/// "-" == -1
/// "=" == -2
/// Eg:
/// 10 == 20
/// 8 == 2=
/// Part A:
/// Sum all the numbers and provide the result in SNAFU

fn from_snafu(from: &str) -> i64 {
    let mut val = 0;
    for unit in from.chars().rev().enumerate() {
        let unit_val = (5 as i64).pow(unit.0 as u32);
        let unit_mult = match unit.1 {
            '-' => -1,
            '=' => -2,
            c => c.to_digit(10).unwrap() as i64,
        };

        val += unit_val * unit_mult;
    }

    val
}

fn to_snafu(from: i64) -> String {
    let mut result = String::from("");

    let mut quot = from;
    while quot > 0 {
        let (q, rem) = (quot + 2).div_mod_floor(&5);
        quot = q;
        let c = match rem - 2 {
            -2 => "=".to_string(),
            -1 => "-".to_string(),
            v => v.to_string(),
        };

        result = c + &result;
    }

    result
}

fn part_a(input: &str) {
    let sum: i64 = input.lines().map(from_snafu).sum();
    let sum_snafu = to_snafu(sum);

    println!("Sum {} == {}", sum, sum_snafu);
}

fn main() {
    let start = time::SystemTime::now();
    let input = include_str!("../../assets/day25.txt");

    part_a(input);

    let end = time::SystemTime::now();
    println!("Took {:?}", end.duration_since(start));
}
