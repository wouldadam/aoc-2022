use std::collections::HashSet;

/// Input is heightmap where a is lowest and z is highest.
/// S indicates start and has height a.
/// E indicates end and has height z.
/// You can only move to a square 1 higher but any number lower.
/// Part A:
/// What is the shortest number of steps to get from start to end.

fn in_bounds(grid: &Vec<Vec<char>>, pos: (usize, usize)) -> bool {
    if pos.0 >= grid.len() {
        return false;
    }

    if pos.1 >= grid[pos.0].len() {
        return false;
    }

    true
}

fn distance(grid: &[Vec<char>], from: (usize, usize), to: (usize, usize)) -> i32 {
    let from_height = grid[from.0][from.1];
    let to_height = grid[to.0][to.1];

    let jump = to_height as i32 - from_height as i32;

    if jump > 1 {
        return i32::MAX;
    }

    1
}

fn dijkstra(grid: Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) {
    let mut unvisited = HashSet::new();
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            unvisited.insert((row, col));
        }
    }

    let mut distances = vec![vec![i32::MAX; grid[0].len()]; grid.len()];
    distances[start.0][start.1] = 0;

    let mut visited = HashSet::new();

    let mut current = start;
    loop {
        let mut dirs = vec![];
        if current.0 != 0 {
            dirs.push((current.0 - 1, current.1));
        }
        dirs.push((current.0 + 1, current.1));
        if current.1 != 0 {
            dirs.push((current.0, current.1 - 1));
        }
        dirs.push((current.0, current.1 + 1));

        for dir in dirs {
            if in_bounds(&grid, dir) {
                let mut dist = distance(&grid, current, dir);
                if dist != i32::MAX {
                    dist += distances[current.0][current.1];
                    distances[dir.0][dir.1] = distances[dir.0][dir.1].min(dist);

                    if dir.0 == end.0 && dir.1 == end.1 {
                        println!("Shortest {}", distances[dir.0][dir.1]);
                        return;
                    }
                }
            }
        }

        unvisited.remove(&current);
        visited.insert(current);

        let next = unvisited
            .iter()
            .filter(|n| distances[n.0][n.1] != i32::MAX)
            .min_by(|a, b| distances[a.0][a.1].cmp(&distances[b.0][b.1]));

        match next {
            Some(n) => current = *n,
            None => unreachable!("No solution"),
        }
    }
}

fn main() {
    let input = include_str!("../../assets/day12.txt");
    let mut grid = input
        .lines()
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut start = (0, 0);
    let mut end = (0, 0);
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 'S' {
                grid[row][col] = 'a';
                start = (row, col);
            }

            if grid[row][col] == 'E' {
                grid[row][col] = 'z';
                end = (row, col);
            }
        }
    }

    dijkstra(grid, start, end);
}
