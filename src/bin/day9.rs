use std::collections::HashSet;

/// You have a rope with a head/tail.
/// The tail must always be touching the head (next to or diagonal to).
/// If you move the head and this is no longer the case, the tail moves.
/// A:
/// Given an input of head movements, count the number of unique spaces visited by the tail.

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Pos {
    x: i64,
    y: i64,
}

#[derive(Clone, Copy)]
enum Step {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let input = include_str!("../../assets/day9.txt");

    // Parse the input moves into individual steps
    let steps = input.lines().flat_map(|line| {
        let seg = line.split(' ').collect::<Vec<_>>();
        let count = seg[1].parse().unwrap();
        let step = match seg[0] {
            "U" => Step::Up,
            "D" => Step::Down,
            "L" => Step::Left,
            "R" => Step::Right,
            _ => panic!("Invalid move direction"),
        };

        let mut steps = vec![];
        for _ in 0..count {
            steps.push(step);
        }

        steps
    });

    let mut head_pos = Pos { x: 0, y: 0 };
    let mut tail_pos = Pos { x: 0, y: 0 };

    let mut tails = HashSet::new();
    tails.insert(tail_pos);

    for step in steps {
        // Move the head
        match step {
            Step::Up => head_pos.y -= 1,
            Step::Down => head_pos.y += 1,
            Step::Left => head_pos.x -= 1,
            Step::Right => head_pos.x += 1,
        }

        // Work out the distance between the H and T
        let diff = Pos {
            x: head_pos.x - tail_pos.x,
            y: head_pos.y - tail_pos.y,
        };

        // If they are too far apart move the tail
        if (diff.x.abs() == 2 || diff.y.abs() == 2) && diff.x.abs() + diff.y.abs() > 2 {
            // Move diag
            tail_pos.x += diff.x.clamp(-1, 1);
            tail_pos.y += diff.y.clamp(-1, 1);
        } else if diff.x.abs() == 2 {
            tail_pos.x += diff.x.clamp(-1, 1);
        } else if diff.y.abs() == 2 {
            tail_pos.y += diff.y.clamp(-1, 1);
        }

        tails.insert(tail_pos);
    }

    println!("Tail position count: {}", tails.len());
}
