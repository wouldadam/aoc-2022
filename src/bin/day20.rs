use std::time;

/// File input is a list of numbers
/// Mixing a list is moving the number in the list by its value.
/// Numbers should be moved in the order they originally appear in th list.
/// The list wraps.
/// Part A:
/// Mix the list then sum the 1000th, 2000th and 3000th numbers after the value 0.
/// Part B:
/// Multiply the numbers by 811589153 and mix the list 10 times.

fn mix(list: &mut Vec<(usize, i64)>, count: usize) {
    for _ in 0..count {
        for orig_idx in 0..list.len() {
            let cur_idx = list
                .iter()
                .position(|(idx, _val)| *idx == orig_idx)
                .unwrap();
            let (_, val) = list[cur_idx];

            let wrap = list.len() as i64 - 1;
            let mut new_idx = cur_idx as i64 + val;
            new_idx = ((new_idx % wrap) + wrap) % wrap;

            move_idx(list, cur_idx, new_idx as usize);
        }
    }
}

fn move_idx(list: &mut [(usize, i64)], from_idx: usize, to_idx: usize) {
    if from_idx < to_idx {
        list[from_idx..=to_idx].rotate_left(1);
    } else {
        list[to_idx..=from_idx].rotate_right(1);
    }
}

fn sum(list: &[(usize, i64)]) {
    let zero_idx = list.iter().position(|e| e.1 == 0).unwrap();
    let v1 = list[(zero_idx + 1000) % list.len()].1;
    let v2 = list[(zero_idx + 2000) % list.len()].1;
    let v3 = list[(zero_idx + 3000) % list.len()].1;
    let sum = v1 + v2 + v3;

    println!("Sum is: {} + {} + {} = {}", v1, v2, v3, sum);
}

fn main() {
    let start = time::SystemTime::now();

    let input = include_str!("../../assets/day20.txt");
    let list = input
        .lines()
        .enumerate()
        .map(|(idx, s)| (idx, s.parse::<i64>().unwrap()))
        .collect::<Vec<_>>();

    let mut part_a = list.clone();
    mix(&mut part_a, 1);
    sum(&part_a);

    let mut part_b = list;
    part_b = part_b
        .iter()
        .map(|(orig_idx, val)| (*orig_idx, *val * 811589153))
        .collect();
    mix(&mut part_b, 10);
    sum(&part_b);

    let end = time::SystemTime::now();
    println!("Took {:?}", end.duration_since(start));
}
