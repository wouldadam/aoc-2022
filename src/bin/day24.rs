use std::{collections::HashSet, ops::Add, time};

use num::integer::lcm;

/// Input describes a grid where "#" is walls, "." is empty ground. "<>^v" are blizzards.
/// Blizzards move in the direction they are point each minute.
/// If a blizzard hits a wall on the opposite side of the grid and starts again.
/// Multiple blizzards can occupy the same position without issue.
/// You start at the empty space on the top row.
/// Your goal is the empty space on the bottom row.
/// You move once per minute simultaneously with blizzards.
/// You cannot share a position with a blizzard.
/// Part A:
/// What is the fewest number of moves to reach the goal?

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    row: i64,
    col: i64,
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Blizzard = (Pos, Direction);

impl Direction {
    fn from(c: char) -> Option<Self> {
        match c {
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            _ => None,
        }
    }

    fn as_char(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    rows: i64,
    cols: i64,

    start: Pos,
    end: Pos,

    blizzards: Vec<Blizzard>,
}

impl Grid {
    fn from(input: &str) -> Self {
        let lines = input.lines().collect::<Vec<_>>();
        let rows = (lines.len() - 2) as i64;
        let cols = (lines[0].len() - 2) as i64;

        let start = Pos {
            row: -1,
            col: (lines[0].chars().position(|c| c == '.').unwrap() - 1) as i64,
        };

        let end = Pos {
            row: rows,
            col: (lines
                .last()
                .unwrap()
                .chars()
                .position(|c| c == '.')
                .unwrap()
                - 1) as i64,
        };

        let mut blizzards = vec![];
        for line in lines[1..lines.len() - 1].iter().enumerate() {
            for c in line.1[1..line.1.len() - 1].chars().enumerate() {
                if let Some(blizzard_dir) = Direction::from(c.1) {
                    blizzards.push((
                        Pos {
                            row: line.0 as i64,
                            col: c.0 as i64,
                        },
                        blizzard_dir,
                    ));
                }
            }
        }

        Grid {
            rows,
            cols,
            start,
            end,
            blizzards,
        }
    }

    fn step_blizzards(&mut self) {
        for blizzard in &mut self.blizzards {
            blizzard.0 = match blizzard.1 {
                Direction::Up => {
                    let mut new_pos = blizzard.0 + Pos { row: -1, col: 0 };
                    if new_pos.row < 0 {
                        new_pos.row = self.rows - 1
                    }

                    new_pos
                }
                Direction::Down => {
                    let mut new_pos = blizzard.0 + Pos { row: 1, col: 0 };
                    if new_pos.row >= self.rows {
                        new_pos.row = 0
                    }

                    new_pos
                }
                Direction::Left => {
                    let mut new_pos = blizzard.0 + Pos { row: 0, col: -1 };
                    if new_pos.col < 0 {
                        new_pos.col = self.cols - 1
                    }

                    new_pos
                }
                Direction::Right => {
                    let mut new_pos = blizzard.0 + Pos { row: 0, col: 1 };
                    if new_pos.col >= self.cols {
                        new_pos.col = 0
                    }

                    new_pos
                }
            };
        }
    }

    fn find_blizzard(&self, pos: Pos) -> Option<&Blizzard> {
        self.blizzards.iter().find(|bliz| bliz.0 == pos)
    }

    fn in_grid(&self, pos: Pos) -> bool {
        if pos == self.start || pos == self.end {
            return true;
        }

        if pos.row < 0 || pos.col < 0 {
            return false;
        }

        if pos.row >= self.rows || pos.col >= self.cols {
            return false;
        }

        true
    }

    #[allow(unused)]
    fn print(&self, pos: Pos) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let cell = Pos { row, col };
                if let Some(blizzard) = self.find_blizzard(cell) {
                    print!("{}", blizzard.1.as_char());
                } else if cell == pos {
                    print!("E");
                } else {
                    print!(".");
                }
            }

            println!();
        }

        println!();
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct State {
    minute: i64,
    pos: Pos,
}

fn solve(start_grid: &Grid) {
    // Pre-compute all grid states
    println!("Computing...");
    let mut grid_states = vec![start_grid.clone()];
    let lcm = lcm(start_grid.rows, start_grid.cols);
    for _ in 0..lcm {
        let mut next = grid_states.last().unwrap().clone();
        next.step_blizzards();
        grid_states.push(next);
    }

    let mut queue = vec![State {
        minute: 0,
        pos: start_grid.start,
    }];
    let steps = vec![
        Pos { row: 1, col: 0 },
        Pos { row: 0, col: 1 },
        Pos { row: 0, col: 0 },
        Pos { row: -1, col: 0 },
        Pos { row: 0, col: -1 },
    ];
    let mut visited = HashSet::new();
    visited.insert(queue[0].clone());

    println!("Solving...");
    while let Some(state) = queue.pop() {
        for step in &steps {
            let new_state = State {
                minute: state.minute + 1,
                pos: state.pos + *step,
            };

            if visited.contains(&new_state) {
                continue;
            }

            let grid = &grid_states[(new_state.minute % lcm) as usize];
            if grid.find_blizzard(new_state.pos).is_none() && grid.in_grid(new_state.pos) {
                if new_state.pos == grid.end {
                    println!("Found end in {} minutes", new_state.minute);
                    return;
                }

                visited.insert(new_state.clone());
                queue.insert(0, new_state);
            }
        }
    }
}

fn main() {
    let start = time::SystemTime::now();
    let input = include_str!("../../assets/day24.txt");

    println!("Loading...");
    let grid = Grid::from(input);

    solve(&grid);

    let end = time::SystemTime::now();
    println!("Took {:?}", end.duration_since(start));
}
