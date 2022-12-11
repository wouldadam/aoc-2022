use std::collections::HashSet;

/// Input is a grid of tree heights.
/// Find the trees visible from the outside looking in.
/// You cannot see over a tree of equal of greater height.

fn find_visible(grid: Vec<Vec<i64>>) {
    let mut set = HashSet::new();

    for (row_idx, row) in grid.iter().enumerate() {
        let mut highest_left = -1;
        let mut highest_right = -1;
        for l_col in 0..row.len() {
            // Check from left to right
            if row[l_col] > highest_left {
                set.insert((row_idx, l_col));
            }

            highest_left = highest_left.max(row[l_col]);

            // Check from right to left
            let r_col = row.len() - l_col - 1;
            if row[r_col] > highest_right {
                set.insert((row_idx, r_col));
            }

            highest_right = highest_right.max(row[r_col]);
        }
    }

    for col in 0..grid[0].len() {
        let mut highest_top = -1;
        let mut highest_bottom = -1;
        for t_row in 0..grid.len() {
            // Check from top to bottom
            if grid[t_row][col] > highest_top {
                set.insert((t_row, col));
            }

            highest_top = highest_top.max(grid[t_row][col]);

            // Check from bottom to top
            let b_row = grid.len() - t_row - 1;
            if grid[b_row][col] > highest_bottom {
                set.insert((b_row, col));
            }

            highest_bottom = highest_bottom.max(grid[b_row][col]);
        }
    }

    println!("Visible trees: {}", set.len());
}

/// The input is a grid of tree heights.
/// You can only see over a tree of it is shorter than the tree you want to look at.
/// You can only look vertically or horizontally.
/// Print how many trees are visible.
fn main() {
    let input = include_str!("../../assets/day8.txt");

    // Parse the input into a grid, indexed [row][col]
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // Print the number of visible trees
    find_visible(grid);
}
