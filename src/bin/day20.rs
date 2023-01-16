use std::time;

/// File input is a list of numbers
/// Mixing a list is moving the number in the list by its value.
/// Numbers should be moved in the order they originally appear in th list.
/// The list wraps.
/// Part A:
/// Mix the list then sum the 1000th, 2000th and 3000th numbers after the value 0.

fn mix(list: &mut Vec<(usize, i32)>) {
    for orig_idx in 0..list.len() {
        let cur_idx = list
            .iter()
            .position(|(idx, _val)| *idx == orig_idx)
            .unwrap();
        let (_, val) = list[cur_idx];

        let wrap = list.len() as i32 - 1;
        let mut new_idx = cur_idx as i32 + val;
        new_idx = ((new_idx % wrap) + wrap) % wrap;

        move_idx(list, cur_idx, new_idx as usize);
    }
}

fn move_idx(list: &mut [(usize, i32)], from_idx: usize, to_idx: usize) {
    if from_idx < to_idx {
        list[from_idx..=to_idx].rotate_left(1);
    } else {
        list[to_idx..=from_idx].rotate_right(1);
    }
}

fn main() {
    let start = time::SystemTime::now();

    let input = include_str!("../../assets/day20.txt");
    let mut list = input
        .lines()
        .enumerate()
        .map(|(idx, s)| (idx, s.parse::<i32>().unwrap()))
        .collect::<Vec<_>>();

    mix(&mut list);

    let zero_idx = list.iter().position(|e| e.1 == 0).unwrap();
    let v1 = list[(zero_idx + 1000) % list.len()].1;
    let v2 = list[(zero_idx + 2000) % list.len()].1;
    let v3 = list[(zero_idx + 3000) % list.len()].1;
    let sum = v1 + v2 + v3;

    println!("Sum is: {} + {} + {} = {}", v1, v2, v3, sum);

    let end = time::SystemTime::now();
    println!("Took {:?}", end.duration_since(start));
}
