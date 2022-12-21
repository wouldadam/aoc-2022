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

const GRID_WIDTH: i32 = 7;
const COL_GAP: i32 = 2;
const ROW_GAP: i32 = 3;
const EMPTY_CELL: char = '.';
const SHAPE_CELL: char = 'X';
const FULL_CELL: char = '#';

#[derive(Clone)]
struct Piece {
    row: i32,
    col: i32,
    shape: Vec<Vec<char>>,
}

impl Piece {
    fn can_place(&self, grid: &Grid, row: i32, col: i32) -> bool {
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

    fn width(&self) -> i32 {
        self.shape[0].len() as i32
    }

    fn height(&self) -> i32 {
        self.shape.len() as i32
    }

    fn new(count: i32, tower_height: i32) -> Self {
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

    fn new_row(tower_height: i32) -> Self {
        Piece {
            row: tower_height + ROW_GAP,
            col: COL_GAP,
            shape: vec![vec![SHAPE_CELL; 4]],
        }
    }

    fn new_plus(tower_height: i32) -> Self {
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

    fn new_l(tower_height: i32) -> Self {
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

    fn new_square(tower_height: i32) -> Self {
        Piece {
            row: tower_height + ROW_GAP,
            col: COL_GAP,
            shape: vec![vec![SHAPE_CELL; 2]; 2],
        }
    }

    fn new_col(tower_height: i32) -> Self {
        Piece {
            row: tower_height + ROW_GAP,
            col: COL_GAP,
            shape: vec![vec![SHAPE_CELL]; 4],
        }
    }
}

#[derive(Clone)]
struct Grid {
    data: Vec<Vec<char>>,
}

impl Grid {
    fn new() -> Self {
        Grid { data: vec![] }
    }

    fn tower_height(&self) -> i32 {
        if self.data.is_empty() {
            return 0;
        }

        for (idx, row) in self.data.iter().enumerate() {
            if !row.contains(&FULL_CELL) {
                return (idx + 1) as i32;
            }
        }

        self.data.len() as i32
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

        println!()
    }
}

fn main() {
    let input = include_str!("../../assets/day17.txt");
    let moves = input.chars().collect::<Vec<_>>();

    // Grid is indexed [row][col], bottom left is [0][0]
    let mut grid = Grid::new();

    let mut move_idx = 0;
    for count in 0..2022 {
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
    }

    grid.print(None);
    println!("Tower height {}", grid.tower_height());
}
