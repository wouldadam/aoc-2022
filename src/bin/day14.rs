/// The input describes x,y coords of bits of rock that sand can rest on.
/// The source of the sand is at 500,0.
/// Sand tries to fall down, then diagonally left, then diagonally right. Then it comes to rest.
/// If there is not rock to hit the sand falls into the abyss forever.
/// Part a:
/// How many units of sand come to rest before the first one falls into the abyss.
/// Part b:
/// There is now a floor 2 units below the lowest rock. How many units of sand until
/// the source is blocked.

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

impl std::ops::Add<Pos> for Pos {
    type Output = Pos;

    fn add(self, rhs: Pos) -> Self::Output {
        Pos {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
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
    let mut grid = vec![vec!['.'; max_pos.col as usize + 500]; max_pos.row as usize + 3];

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
        for col in &row[450..505] {
            print!("{}", col);
        }
        println!();
    }
}

fn run_till_abyss(grid: &mut [Vec<char>]) -> usize {
    let mut count = 0;

    loop {
        let mut sand_pos = SOURCE_POS;

        loop {
            let down = sand_pos + Pos::new(1, 0);
            let left_down = sand_pos + Pos::new(1, -1);
            let right_down = sand_pos + Pos::new(1, 1);

            let mut moved = false;
            for mv in [down, left_down, right_down] {
                // OOB
                if mv.row < 0
                    || mv.row >= grid.len() as i32
                    || mv.col < 0
                    || mv.col >= grid[0].len() as i32
                {
                    continue;
                }

                // Rock or sand
                if grid[mv.row as usize][mv.col as usize] != '.' {
                    continue;
                }

                sand_pos = mv;
                moved = true;
                break;
            }

            if sand_pos.row + 1 == grid.len() as i32 {
                return count;
            }

            if sand_pos == SOURCE_POS {
                return count;
            }

            if !moved {
                grid[sand_pos.row as usize][sand_pos.col as usize] = 'o';
                count += 1;
                break;
            }
        }
    }
}

fn main() {
    let input = include_str!("../../assets/day14.txt");
    let (seams, max_pos) = load_seams(input);

    let mut a_grid = build_grid(&seams, &max_pos);
    let units_till_abyss = run_till_abyss(&mut a_grid);
    println!("Units of sand till abyss {}", units_till_abyss);

    let mut b_grid = build_grid(&seams, &max_pos);
    for c in b_grid.last_mut().unwrap() {
        *c = '#';
    }

    let units_till_blocked = run_till_abyss(&mut b_grid);
    println!("Units of sand till blocked {}", units_till_blocked + 1);
}
