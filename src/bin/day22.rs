use std::time;

use regex::Regex;

/// Input is split in two.
/// First half is a grid map where "." is a passable and "#" is a wall.
/// Second half is a path to follow.
/// A number is the number of tiles to move in the direction you are facing.
/// A letter indicates a 90deg turn clockwise (R) or anticlockwise (L).
/// The start position is the top-leftmost passable tile facing to the right.
/// A move off the map wraps around the map.
/// If you hit a wall you stop and continue with the next instruction.
/// Part A:
/// The password is based on your final position: (1000 * row) + (4* col) * f.
/// f is the direction faced where: right=0, down=1, left=2, up=3.
/// row and col start at 1.
/// Find the password.

#[derive(Debug)]
enum Rotation {
    Clockwise,
    AntiClockwise,
}

impl Rotation {
    fn from(input: &str) -> Self {
        match input {
            "R" => Rotation::Clockwise,
            "L" => Rotation::AntiClockwise,
            _ => panic!("Invalid rotation."),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Facing {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Facing {
    fn rotate(&mut self, turn: &Rotation) {
        let by = match turn {
            Rotation::Clockwise => 1,
            Rotation::AntiClockwise => -1,
        };

        *self = Facing::from(((*self as i32) + by).rem_euclid(4));
    }

    fn from(val: i32) -> Self {
        match val {
            0 => Facing::Right,
            1 => Facing::Down,
            2 => Facing::Left,
            3 => Facing::Up,
            _ => panic!("Invalid facing: {}", val),
        }
    }
}

#[derive(Debug)]
enum Step {
    Turn(Rotation),
    Forward,
}

#[derive(Debug, Clone, PartialEq)]
enum Tile {
    Void,
    Passable,
    Wall,
}

impl Tile {
    fn from(input: &char) -> Self {
        match input {
            ' ' => Tile::Void,
            '.' => Tile::Passable,
            '#' => Tile::Wall,
            _ => panic!("Invalid tile."),
        }
    }
}

#[derive(Debug)]
struct Map {
    grid: Vec<Vec<Tile>>,
    steps: Vec<Step>,
    pos: (i32, i32, Facing),
}

impl Map {
    fn from(input: &str) -> Self {
        let halves = input.split("\n\n").collect::<Vec<_>>();
        if halves.len() != 2 {
            panic!("Invalid input");
        }

        // Parse the grid
        let col_max = halves[0].lines().max_by_key(|row| row.len()).unwrap().len();
        let mut grid = vec![];
        for row_str in halves[0].lines() {
            let mut row = vec![Tile::Void; col_max];
            for (idx, tile_str) in row_str.chars().enumerate() {
                row[idx] = Tile::from(&tile_str);
            }

            grid.push(row);
        }

        // Parse the steps
        let steps_str = halves[1];
        let step_re = Regex::new(r"(\d*)([LR])?").unwrap();

        let mut steps = vec![];
        for caps in step_re.captures_iter(steps_str) {
            for _ in 0..caps[1].parse::<usize>().unwrap() {
                steps.push(Step::Forward);
            }

            if let Some(turn) = caps.get(2) {
                steps.push(Step::Turn(Rotation::from(turn.as_str())));
            }
        }

        // Find the start position
        let start_col = grid[0]
            .iter()
            .position(|tile| *tile == Tile::Passable)
            .unwrap() as i32;
        let pos = (0, start_col, Facing::Right);

        Map { grid, steps, pos }
    }

    fn at(&self, row_idx: i32) -> &Vec<Tile> {
        &self.grid[row_idx as usize]
    }

    fn at_row(&self, row_idx: i32, col_idx: i32) -> &Tile {
        &self.grid[row_idx as usize][col_idx as usize]
    }

    fn at_pos(&self, pos: (i32, i32, Facing)) -> &Tile {
        &self.grid[pos.0 as usize][pos.1 as usize]
    }

    fn run(&mut self) {
        for step in &self.steps {
            if let Step::Forward = step {
                let new_pos = match self.pos.2 {
                    Facing::Right => {
                        let mut new_col = self.pos.1 + 1;
                        if new_col as usize >= self.at(self.pos.0).len()
                            || *self.at_row(self.pos.0, new_col) == Tile::Void
                        {
                            new_col = self
                                .at(self.pos.0)
                                .iter()
                                .position(|tile| *tile != Tile::Void)
                                .unwrap() as i32;
                        }

                        (self.pos.0, new_col, self.pos.2)
                    }
                    Facing::Down => {
                        let mut new_row = self.pos.0 + 1;
                        if new_row as usize >= self.grid.len()
                            || *self.at_row(new_row, self.pos.1) == Tile::Void
                        {
                            new_row = self
                                .grid
                                .iter()
                                .position(|row| row[0] != Tile::Void)
                                .unwrap() as i32;
                        }

                        (new_row, self.pos.1, self.pos.2)
                    }
                    Facing::Left => {
                        let mut new_col = self.pos.1 - 1;
                        if new_col < 0 || *self.at_row(self.pos.0, new_col) == Tile::Void {
                            new_col = self.at(self.pos.0).len() as i32
                                - self
                                    .at(self.pos.0)
                                    .iter()
                                    .rev()
                                    .position(|tile| *tile != Tile::Void)
                                    .unwrap() as i32
                                - 1;
                        }

                        (self.pos.0, new_col, self.pos.2)
                    }
                    Facing::Up => {
                        let mut new_row = self.pos.0 - 1;
                        if new_row < 0 || *self.at_row(new_row, self.pos.1) == Tile::Void {
                            new_row = self.grid.len() as i32
                                - self
                                    .grid
                                    .iter()
                                    .rev()
                                    .position(|row| row[0] != Tile::Void)
                                    .unwrap() as i32
                                - 1;
                        }

                        (new_row, self.pos.1, self.pos.2)
                    }
                };

                if *self.at_pos(new_pos) == Tile::Passable {
                    self.pos = new_pos;
                }
            } else if let Step::Turn(turn) = step {
                self.pos.2.rotate(turn);
            }
        }
    }

    fn password(&self) -> i32 {
        (1000 * (self.pos.0 + 1)) + (4 * (self.pos.1 + 1)) + (self.pos.2 as i32)
    }
}

fn main() {
    let start = time::SystemTime::now();
    let input = include_str!("../../assets/day22.txt");

    let mut map = Map::from(input);
    println!("Start: {:?}", map);

    println!("Running...");
    map.run();
    println!("End: {:?}", map);

    println!("Password: {}", map.password());

    let end = time::SystemTime::now();
    println!("Took {:?}", end.duration_since(start));
}
