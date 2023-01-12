use std::collections::HashSet;

/// The input defines the x,y,z coord of 1x1x1 cubes in 3d space
/// Part A:
/// Count the number of exposed sides of cubes.
/// Part B:
/// Count the number of exposed sides of cubes, excluding cavities.

type Pos = (i32, i32, i32);

fn count_exposed_sides(cubes: &HashSet<Pos>) {
    let mut count = 0;

    for (x, y, z) in cubes {
        let left = (x - 1, *y, *z);
        let right = (x + 1, *y, *z);
        let top = (*x, y - 1, *z);
        let bottom = (*x, y + 1, *z);
        let forward = (*x, *y, z + 1);
        let backward = (*x, *y, z - 1);

        let checks = [left, right, top, bottom, forward, backward];

        for check in checks {
            if !cubes.contains(&check) {
                count += 1;
            }
        }
    }

    println!("Exposed sides: {}", count);
}

fn fill_cavities(cubes: &HashSet<Pos>) -> HashSet<Pos> {
    // Find bounding box +1
    let mut min = (0, 0, 0);
    let mut max = (0, 0, 0);
    for (x, y, z) in cubes {
        min.0 = min.0.min(*x);
        min.1 = min.1.min(*y);
        min.2 = min.2.min(*z);

        max.0 = max.0.max(*x);
        max.1 = max.1.max(*y);
        max.2 = max.2.max(*z);
    }

    min.0 -= 1;
    min.1 -= 1;
    min.2 -= 1;

    max.0 += 1;
    max.1 += 1;
    max.2 += 1;

    // Now flood fill inside the bounding box
    let mut steam = HashSet::new();
    let mut stack = vec![min];
    while let Some((x, y, z)) = stack.pop() {
        steam.insert((x, y, z));

        let left = (x - 1, y, z);
        let right = (x + 1, y, z);
        let top = (x, y - 1, z);
        let bottom = (x, y + 1, z);
        let forward = (x, y, z + 1);
        let backward = (x, y, z - 1);

        let moves = [left, right, top, bottom, forward, backward];

        for mv in moves {
            if in_bounding_box(&mv, &min, &max) && !steam.contains(&mv) && !cubes.contains(&mv) {
                stack.push(mv);
            }
        }
    }

    // Everything inside the bounding box that isn't a cube and isn't steam is a cavity.
    // Combine cavity and cubes.
    let mut filled = cubes.clone();
    for x in min.0..max.0 {
        for y in min.1..max.1 {
            for z in min.2..max.2 {
                let pos = (x, y, z);
                if !cubes.contains(&pos) && !steam.contains(&pos) {
                    filled.insert(pos);
                }
            }
        }
    }

    filled
}

fn in_bounding_box(pos: &Pos, min: &Pos, max: &Pos) -> bool {
    if pos.0 < min.0 || pos.0 > max.0 {
        return false;
    }

    if pos.1 < min.1 || pos.1 > max.1 {
        return false;
    }

    if pos.2 < min.2 || pos.2 > max.2 {
        return false;
    }

    true
}

fn main() {
    let input = include_str!("../../assets/day18.txt");

    let cubes = input
        .lines()
        .map(|line| {
            let els = line
                .split(',')
                .map(|ele| ele.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            (els[0], els[1], els[2])
        })
        .collect::<HashSet<_>>();

    // Part A
    count_exposed_sides(&cubes);

    // Part B
    let filled = fill_cavities(&cubes);
    count_exposed_sides(&filled);
}
