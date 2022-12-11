use std::collections::HashSet;

/// You have a rope with a head/tail.
/// The tail must always be touching the head (next to or diagonal to).
/// If you move the head and this is no longer the case, the tail moves.
/// A:
/// Given an input of head movements, count the number of unique spaces visited by the tail.
/// A:
/// Increase the number of knots to 10 and calculate the same count.

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

fn count_tail_pos(steps: &Vec<Step>, knots: usize) {
    let mut rope = vec![Pos { x: 0, y: 0 }; knots];

    let mut tails = HashSet::new();
    tails.insert(*rope.last().unwrap());

    for step in steps {
        // Move the head
        match step {
            Step::Up => rope[0].y -= 1,
            Step::Down => rope[0].y += 1,
            Step::Left => rope[0].x -= 1,
            Step::Right => rope[0].x += 1,
        }

        for idx in 1..rope.len() {
            let prev = rope[idx - 1];
            let curr = &mut rope[idx];
            // Work out the distance between the previous and current knot
            let diff = Pos {
                x: prev.x - curr.x,
                y: prev.y - curr.y,
            };

            // If they are too far apart move the knot
            if (diff.x.abs() == 2 || diff.y.abs() == 2) && diff.x.abs() + diff.y.abs() > 2 {
                // Move diag
                curr.x += diff.x.clamp(-1, 1);
                curr.y += diff.y.clamp(-1, 1);
            } else if diff.x.abs() == 2 {
                curr.x += diff.x.clamp(-1, 1);
            } else if diff.y.abs() == 2 {
                curr.y += diff.y.clamp(-1, 1);
            }
        }

        tails.insert(*rope.last().unwrap());
    }

    println!("Tail position count: {}", tails.len()); // 6357
}

fn main() {
    let input = include_str!("../../assets/day9.txt");

    // Parse the input moves into individual steps
    let steps = input
        .lines()
        .flat_map(|line| {
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
        })
        .collect::<Vec<_>>();

    count_tail_pos(&steps, 2);

    count_tail_pos(&steps, 10);
}
