use std::{fmt::Display, time};

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
/// Part B:
/// The map is not longer a flat map. Instead the map now describes the net for faces of a 3d cube.
/// This changes the wrapping rules so that you "walk" around the cube.
/// All the other rules are the same.
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

#[derive(Debug, Clone, Copy, PartialEq)]
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

type Pos = (i32, i32, Facing);

#[derive(Debug, Clone, PartialEq)]
enum Tile {
    Void,
    Passable,
    Wall,
    Tunnel(Pos, Pos),
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
    pos: Pos,
    has_tunnels: bool,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (row_idx, row) in self.grid.iter().enumerate() {
            for (col_idx, tile) in row.iter().enumerate() {
                if (row_idx, col_idx) == (self.pos.0 as usize, self.pos.1 as usize) {
                    write!(f, "O")?
                } else {
                    match tile {
                        Tile::Void => write!(f, " ")?,
                        Tile::Passable => write!(f, ".")?,
                        Tile::Wall => write!(f, "#")?,
                        Tile::Tunnel(_, _) => write!(f, "T")?,
                    }
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
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

        Map {
            grid,
            steps,
            pos,
            has_tunnels: false,
        }
    }

    fn add_cube_tunnels(&mut self, net: i32) {
        let new_width = self.grid[0].len() + 2;

        for row in &mut self.grid {
            row.insert(0, Tile::Void);
            row.push(Tile::Void);
        }

        self.grid.insert(0, vec![Tile::Void; new_width]);
        self.grid.push(vec![Tile::Void; new_width]);

        self.pos = (self.pos.0 + 1, self.pos.1 + 1, self.pos.2);
        self.has_tunnels = true;

        // Nets are hard coded - hack
        if net == 0 {
            //  1
            //234
            //  56
            for spot in 1..5 {
                // Top of 1 to top of 2
                let top_1 = spot + 8;
                let top_2 = 5 - spot;
                self.grid[0][top_1] = Tile::Tunnel(
                    (0, top_1 as i32, Facing::Down),
                    (4, top_2 as i32, Facing::Down),
                );
                self.grid[4][top_2] = self.grid[0][top_1].clone();

                // Top of 3 to left side of 1
                let top_3 = spot + 4;
                let left_1 = spot;
                self.grid[4][top_3] = Tile::Tunnel(
                    (4, top_3 as i32, Facing::Down),
                    (left_1 as i32, 8, Facing::Right),
                );
                self.grid[left_1][8] = self.grid[4][top_3].clone();

                // Right of 1 to right of 6
                let right_1 = spot;
                let right_6 = 13 - spot;
                self.grid[right_1][13] = Tile::Tunnel(
                    (right_1 as i32, 13, Facing::Left),
                    (right_6 as i32, 13, Facing::Left),
                );
                self.grid[right_6][17] = self.grid[right_1][13].clone();

                // Left of 2 to bottom of 6
                let left_2 = spot + 4;
                let bottom_6 = 17 - spot;
                self.grid[left_2][0] = Tile::Tunnel(
                    (left_2 as i32, 0, Facing::Right),
                    (13, bottom_6 as i32, Facing::Up),
                );
                self.grid[13][bottom_6] = self.grid[left_2][0].clone();

                // Bottom of 2 to bottom of 5
                let bottom_2 = spot;
                let bottom_5 = 13 - spot;
                self.grid[9][bottom_2] = Tile::Tunnel(
                    (9, bottom_2 as i32, Facing::Up),
                    (13, bottom_5 as i32, Facing::Up),
                );
                self.grid[13][bottom_5] = self.grid[9][bottom_2].clone();

                // Bottom of 3 to left of 5
                let bottom_3 = spot + 4;
                let left_5 = 13 - spot;
                self.grid[9][bottom_3] = Tile::Tunnel(
                    (9, bottom_3 as i32, Facing::Up),
                    (left_5 as i32, 8, Facing::Right),
                );
                self.grid[left_5][8] = self.grid[9][bottom_3].clone();

                // Right of 4 to top of 6
                let right_4 = spot + 4;
                let top_6 = 17 - spot;
                self.grid[right_4][13] = Tile::Tunnel(
                    (right_4 as i32, 13, Facing::Left),
                    (8, top_6 as i32, Facing::Down),
                );
                self.grid[8][top_6] = self.grid[right_4][13].clone();
            }
        } else if net == 1 {
            // 12
            // 3
            //45
            //6
            for spot in 1..51 {
                // Top of 1 to left of 6
                let top_1 = 50 + spot;
                let left_6 = 150 + spot;
                self.grid[0][top_1] = Tile::Tunnel(
                    (0, top_1 as i32, Facing::Down),
                    (left_6 as i32, 0, Facing::Right),
                );
                self.grid[left_6][0] = self.grid[0][top_1].clone();

                // Left of 1 to left of 4
                let left_1 = spot;
                let left_4 = 151 - spot;
                self.grid[left_1][50] = Tile::Tunnel(
                    (left_1 as i32, 50, Facing::Right),
                    (left_4 as i32, 0, Facing::Right),
                );
                self.grid[left_4][0] = self.grid[left_1][50].clone();

                // Top of 2 to bottom of 6
                let top_2 = 100 + spot;
                let bottom_6 = spot;
                self.grid[0][top_2] = Tile::Tunnel(
                    (0, top_2 as i32, Facing::Down),
                    (201, bottom_6 as i32, Facing::Up),
                );
                self.grid[201][bottom_6] = self.grid[0][top_2].clone();

                // Right of 2 to right of 5
                let right_2 = spot;
                let right_5 = 151 - spot;
                self.grid[right_2][151] = Tile::Tunnel(
                    (right_2 as i32, 151, Facing::Left),
                    (right_5 as i32, 101, Facing::Left),
                );
                self.grid[right_5][101] = self.grid[right_2][151].clone();

                // Bottom of 2 to right of 3
                let bottom_2 = 100 + spot;
                let right_3 = 50 + spot;
                self.grid[51][bottom_2] = Tile::Tunnel(
                    (51, bottom_2 as i32, Facing::Up),
                    (right_3 as i32, 101, Facing::Left),
                );
                self.grid[right_3][101] = self.grid[51][bottom_2].clone();

                // Left of 3 to top of 4
                let left_3 = 50 + spot;
                let top_4 = spot;
                self.grid[left_3][50] = Tile::Tunnel(
                    (left_3 as i32, 50, Facing::Right),
                    (100, top_4 as i32, Facing::Down),
                );
                self.grid[100][top_4] = self.grid[left_3][50].clone();

                // Bottom of 5 to right of 6
                let bottom_5 = 50 + spot;
                let right_6 = 150 + spot;
                self.grid[151][bottom_5] = Tile::Tunnel(
                    (151, bottom_5 as i32, Facing::Up),
                    (right_6 as i32, 51, Facing::Left),
                );
                self.grid[right_6][51] = self.grid[151][bottom_5].clone();
            }
        } else {
            panic!("Unknown net");
        }
    }

    fn at(&self, row_idx: i32) -> &Vec<Tile> {
        &self.grid[row_idx as usize]
    }

    fn at_row(&self, row_idx: i32, col_idx: i32) -> &Tile {
        &self.grid[row_idx as usize][col_idx as usize]
    }

    fn at_pos(&self, pos: Pos) -> &Tile {
        &self.grid[pos.0 as usize][pos.1 as usize]
    }

    fn ride(&self, from: Pos) -> Pos {
        if let Tile::Tunnel(point_a, point_b) = self.at_row(from.0, from.1) {
            if point_a.0 == from.0 && point_a.1 == from.1 {
                return self.peek_forward(*point_b);
            } else {
                return self.peek_forward(*point_a);
            }
        }

        from
    }

    fn peek_forward(&self, from: Pos) -> Pos {
        let peek_pos = match from.2 {
            Facing::Right => {
                let mut new_col = from.1 + 1;
                if new_col as usize >= self.at(from.0).len()
                    || *self.at_row(from.0, new_col) == Tile::Void
                {
                    new_col = self
                        .at(from.0)
                        .iter()
                        .position(|tile| *tile != Tile::Void)
                        .unwrap() as i32;
                }

                (from.0, new_col, from.2)
            }
            Facing::Down => {
                let mut new_row = from.0 + 1;
                if new_row as usize >= self.grid.len()
                    || *self.at_row(new_row, from.1) == Tile::Void
                {
                    new_row = self
                        .grid
                        .iter()
                        .position(|row| row[0] != Tile::Void)
                        .unwrap() as i32;
                }

                (new_row, from.1, from.2)
            }
            Facing::Left => {
                let mut new_col = from.1 - 1;
                if new_col < 0 || *self.at_row(from.0, new_col) == Tile::Void {
                    new_col = self.at(from.0).len() as i32
                        - self
                            .at(from.0)
                            .iter()
                            .rev()
                            .position(|tile| *tile != Tile::Void)
                            .unwrap() as i32
                        - 1;
                }

                (from.0, new_col, from.2)
            }
            Facing::Up => {
                let mut new_row = from.0 - 1;
                if new_row < 0 || *self.at_row(new_row, from.1) == Tile::Void {
                    new_row = self.grid.len() as i32
                        - self
                            .grid
                            .iter()
                            .rev()
                            .position(|row| row[0] != Tile::Void)
                            .unwrap() as i32
                        - 1;
                }

                (new_row, from.1, from.2)
            }
        };

        peek_pos
    }

    fn run(&mut self) {
        for step in &self.steps {
            if let Step::Forward = step {
                let mut new_pos = self.peek_forward(self.pos);
                new_pos = self.ride(new_pos);

                if *self.at_pos(new_pos) == Tile::Passable {
                    self.pos = new_pos;
                }
            } else if let Step::Turn(turn) = step {
                self.pos.2.rotate(turn);
            }
        }
    }

    fn password(&self) -> i32 {
        let off_by = if self.has_tunnels { 0 } else { 1 };
        (1000 * (self.pos.0 + off_by)) + (4 * (self.pos.1 + off_by)) + (self.pos.2 as i32)
    }
}

fn main() {
    let start = time::SystemTime::now();
    let input = include_str!("../../assets/day22.txt");

    // Part A
    {
        let mut map = Map::from(input);
        println!("Running flat...");
        map.run();

        println!("Password: {}", map.password());
    }

    println!();

    // Part B
    {
        let mut map = Map::from(input);
        map.add_cube_tunnels(1);
        println!("{}", map);
        println!("Running cube...");
        map.run();

        println!("Password: {}", map.password());
    }

    println!();

    let end = time::SystemTime::now();
    println!("Took {:?}", end.duration_since(start));
}
