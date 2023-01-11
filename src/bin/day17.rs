use std::collections::HashMap;

/// Rocks are falling with the familiar shapes of:
/// ####
///
/// .#.
/// ###
/// .#.
///
/// ..#
/// ..#
/// ###
///
/// #
/// #
/// #
/// #
///
/// ##
/// ##
///
/// They fall in the repeating order above.
/// The input describes air pushing the rocks left (<) and right (>).
/// The room is 7 units wide.
/// Each new rock appears so its left edge is 2 units from the left wall
/// and 3 units above the highest rock or floor.
/// A rock is push as soon as it appears and before it falls the first unit.
/// Part A:
/// How tall will the tower of rocks be after 2022 rocks have fallen?

const GRID_WIDTH: i64 = 7;
const COL_GAP: i64 = 2;
const ROW_GAP: i64 = 3;
const EMPTY_CELL: char = '.';
const SHAPE_CELL: char = 'X';
const FULL_CELL: char = '#';

#[derive(Clone)]
struct Piece {
    row: i64,
    col: i64,
    shape: Vec<Vec<char>>, // indexed row,col
}

impl Piece {
    fn can_place(&self, grid: &Grid, row: i64, col: i64) -> bool {
        // The piece is outside the side of the grid
        if !(0..GRID_WIDTH + 1).contains(&col)
            || !(0..GRID_WIDTH + 1).contains(&(col + self.width()))
        {
            return false;
        }

        // The piece is off the bottom of the grid
        if row < 0 {
            return false;
        }

        // The piece is higher than the tower so can't hit anything
        if row >= grid.tower_height() {
            return true;
        }

        // Check all cells in the grid
        for piece_row_idx in 0..self.shape.len() {
            for piece_col_idx in 0..self.shape[piece_row_idx].len() {
                let grid_row_idx = piece_row_idx + row as usize;
                let grid_col_idx = piece_col_idx + col as usize;

                if grid_row_idx >= grid.data.len() {
                    continue;
                }
                if self.shape[piece_row_idx][piece_col_idx] == SHAPE_CELL
                    && grid.data[grid_row_idx][grid_col_idx] == FULL_CELL
                {
                    return false;
                }
            }
        }

        true
    }

    fn width(&self) -> i64 {
        self.shape[0].len() as i64
    }

    fn height(&self) -> i64 {
        self.shape.len() as i64
    }

    fn new(count: i64, tower_height: i64) -> Self {
        let idx = count % 5;

        if idx == 0 {
            return Piece::new_row(tower_height);
        } else if idx == 1 {
            return Piece::new_plus(tower_height);
        } else if idx == 2 {
            return Piece::new_l(tower_height);
        } else if idx == 3 {
            return Piece::new_col(tower_height);
        } else if idx == 4 {
            return Piece::new_square(tower_height);
        }

        unreachable!("This shouldn't happen");
    }

    fn new_row(tower_height: i64) -> Self {
        Piece {
            row: tower_height + ROW_GAP,
            col: COL_GAP,
            shape: vec![vec![SHAPE_CELL; 4]],
        }
    }

    fn new_plus(tower_height: i64) -> Self {
        Piece {
            row: tower_height + ROW_GAP,
            col: COL_GAP,
            shape: vec![
                vec![EMPTY_CELL, SHAPE_CELL, EMPTY_CELL],
                vec![SHAPE_CELL, SHAPE_CELL, SHAPE_CELL],
                vec![EMPTY_CELL, SHAPE_CELL, EMPTY_CELL],
            ],
        }
    }

    fn new_l(tower_height: i64) -> Self {
        Piece {
            row: tower_height + ROW_GAP,
            col: COL_GAP,
            shape: vec![
                vec![SHAPE_CELL, SHAPE_CELL, SHAPE_CELL],
                vec![EMPTY_CELL, EMPTY_CELL, SHAPE_CELL],
                vec![EMPTY_CELL, EMPTY_CELL, SHAPE_CELL],
            ],
        }
    }

    fn new_square(tower_height: i64) -> Self {
        Piece {
            row: tower_height + ROW_GAP,
            col: COL_GAP,
            shape: vec![vec![SHAPE_CELL; 2]; 2],
        }
    }

    fn new_col(tower_height: i64) -> Self {
        Piece {
            row: tower_height + ROW_GAP,
            col: COL_GAP,
            shape: vec![vec![SHAPE_CELL]; 4],
        }
    }
}

#[derive(Clone)]
struct Grid {
    // indexed row,col
    data: Vec<Vec<char>>,
}

impl Grid {
    fn new() -> Self {
        Grid { data: vec![] }
    }

    fn tower_height(&self) -> i64 {
        if self.data.is_empty() {
            return 0;
        }

        for (idx, row) in self.data.iter().enumerate() {
            if !row.contains(&FULL_CELL) {
                return (idx + 1) as i64;
            }
        }

        self.data.len() as i64
    }

    fn place(&mut self, piece: &Piece) {
        let new_height = piece.row as usize + piece.shape.len();
        if new_height > self.data.len() {
            self.data.extend(vec![
                vec![EMPTY_CELL; GRID_WIDTH as usize];
                new_height - self.data.len()
            ]);
        }

        let (mut piece_row, mut piece_col) = (0, 0);

        for grid_row in piece.row..piece.row + piece.height() {
            for grid_col in piece.col..piece.col + piece.width() {
                if piece.shape[piece_row][piece_col] == SHAPE_CELL {
                    self.data[grid_row as usize][grid_col as usize] = FULL_CELL;
                }

                piece_col += 1;
            }

            piece_row += 1;
            piece_col = 0;
        }
    }

    #[allow(dead_code)]
    fn print(&self, piece: Option<Piece>) {
        let mut placed = self.clone();
        if let Some(p) = piece {
            placed.place(&p)
        }

        for row in placed.data.iter().rev() {
            for col in row {
                print!("{}", &col);
            }
            println!();
        }

        println!();
    }
}

fn run(moves: &Vec<char>, rocks: i64) {
    let mut grid = Grid::new();

    let mut cycles = HashMap::new();
    let mut from_cycle = 0;

    let mut move_idx = 0;
    let mut count = 0;
    while count < rocks {
        let mut piece = Piece::new(count, grid.tower_height());

        loop {
            let (gust_row, gust_col) = match moves[move_idx] {
                '>' => (piece.row, piece.col + 1),
                '<' => (piece.row, piece.col - 1),
                _ => panic!("Invalid move"),
            };
            move_idx = (move_idx + 1) % moves.len();

            if piece.can_place(&grid, gust_row, gust_col) {
                piece.row = gust_row;
                piece.col = gust_col;
            }

            let (move_row, move_col) = (piece.row - 1, piece.col);
            if piece.can_place(&grid, move_row, move_col) {
                piece.row = move_row;
                piece.col = move_col;
            } else {
                grid.place(&piece);
                break;
            }
        }

        if from_cycle == 0 {
            let piece_idx = count % 5;
            let key = (piece_idx, move_idx);

            if let Some((2, prev_count, prev_top)) = cycles.get(&key) {
                println!(
                    "Found cycle at rock {}, height {}",
                    count,
                    grid.tower_height()
                );

                let count_diff = count - prev_count + 1;
                let top_diff = grid.tower_height() - prev_top;
                let repeats = (rocks - count) / count_diff;

                from_cycle = repeats * top_diff;
                count += repeats * count_diff;
                count += 1;
                println!("Skipping {} height to rock {}", from_cycle, count);
            } else {
                count += 1;
            }

            cycles
                .entry(key)
                .and_modify(|(occurrence, prev_count, prev_top)| {
                    *occurrence += 1;
                    *prev_count = count;
                    *prev_top = grid.tower_height();
                })
                .or_insert((1, count, grid.tower_height()));
        } else {
            count += 1;
        }
    }

    println!("Tower height {}", grid.tower_height() + from_cycle);
}

fn main() {
    let input = include_str!("../../assets/day17.txt");
    let moves = input.chars().collect::<Vec<_>>();

    run(&moves, 2022);
    run(&moves, 1_000_000_000_000);
}
