/// The input describes x,y coords of bits of rock that sand can rest on.
/// The source of the sand is at 500,0.
/// Sand tries to fall down, then diagonally left, then diagonally right. Then it comes to rest.
/// If there is not rock to hit the sand falls into the abyss forever.
/// Part a:
/// How many units of sand come to rest before the first one falls into the abyss.

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Pos {
    row: i32,
    col: i32,
}

impl Pos {
    fn new(row: i32, col: i32) -> Pos {
        Pos { row, col }
    }
}

impl std::ops::AddAssign<Pos> for Pos {
    fn add_assign(&mut self, rhs: Pos) {
        self.row += rhs.row;
        self.col += rhs.col;
    }
}

const SOURCE_POS: Pos = Pos { row: 0, col: 500 };

/// Parse input into rock seams
fn load_seams(input: &str) -> (Vec<Vec<Pos>>, Pos) {
    let mut max_row = 0;
    let mut max_col = 0;
    let seams = input
        .lines()
        .map(|line| {
            line.split("->")
                .map(|pos| {
                    let coords = pos
                        .split(',')
                        .map(|c| c.trim().parse::<i32>().unwrap())
                        .collect::<Vec<_>>();

                    max_row = max_row.max(coords[1]);
                    max_col = max_col.max(coords[0]);

                    Pos::new(coords[1], coords[0])
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (seams, Pos::new(max_row, max_col))
}

/// Build the grid, '.'=empty, '#'=rock
fn build_grid(seams: &Vec<Vec<Pos>>, max_pos: &Pos) -> Vec<Vec<char>> {
    let mut grid = vec![vec!['.'; max_pos.col as usize + 1]; max_pos.row as usize + 1];

    // Add the seams
    for seam in seams {
        for points in seam.windows(2) {
            let from = &points[0];
            let to = &points[1];

            let diff = Pos::new(
                (to.row - from.row).clamp(-1, 1),
                (to.col - from.col).clamp(-1, 1),
            );

            let mut current = *from;
            while current != *to {
                grid[current.row as usize][current.col as usize] = '#';
                current += diff;
            }
            grid[current.row as usize][current.col as usize] = '#';
        }
    }

    grid
}

/// Print the grid to console
#[allow(dead_code)]
fn print_grid(grid: &[Vec<char>]) {
    for row in grid {
        for col in &row[450..504] {
            print!("{}", col);
        }
        println!();
    }
}

fn run_till_abyss(grid: &mut [Vec<char>], max_pos: &Pos) -> usize {
    let mut count = 0;

    loop {
        let mut sand_pos = SOURCE_POS;

        loop {
            if sand_pos.row < max_pos.row
                && grid[sand_pos.row as usize + 1][sand_pos.col as usize] == '.'
            {
                // Down
                sand_pos += Pos::new(1, 0);
            } else if sand_pos.row < max_pos.row
                && sand_pos.col > 0
                && grid[sand_pos.row as usize + 1][sand_pos.col as usize - 1] == '.'
            {
                // Down and left
                sand_pos += Pos::new(1, -1);
            } else if sand_pos.row < max_pos.row
                && sand_pos.col < max_pos.col
                && grid[sand_pos.row as usize + 1][sand_pos.col as usize + 1] == '.'
            {
                // Down and right
                sand_pos += Pos::new(1, 1);
            } else {
                // At rest
                grid[sand_pos.row as usize][sand_pos.col as usize] = 'o';
                count += 1;
                break;
            }

            // We are in the abyss
            if sand_pos.row >= max_pos.row {
                return count;
            }
        }
    }
}

fn main() {
    let input = include_str!("../../assets/day14.txt");
    let (seams, max_pos) = load_seams(input);
    let mut grid = build_grid(&seams, &max_pos);
    print_grid(&grid);

    let units_till_abyss = run_till_abyss(&mut grid, &max_pos);

    println!("Units of sand till abyss {}", units_till_abyss);
}
