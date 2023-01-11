use std::collections::HashSet;

/// The input defines the x,y,z coord of 1x1x1 cubes in 3d space
/// Part A:
/// Count the number of exposed sides of cubes.

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

    count_exposed_sides(&cubes);
}
