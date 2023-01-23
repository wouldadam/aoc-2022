use std::{
    collections::{HashMap, HashSet},
    ops::Add,
    time,
};

/// Input shows a grid of elves "#" and empty ground ".".
/// Empty ground expands around this grid in every direction.
/// Elves move in rounds. In the first half of a round each elf
/// picks a direction with the following rules:
///  if no adjacent elves - do nothing
///  if no elves in N/NE/NW - propose N
///  if no elves in S/SE/SW - propose S
///  if no elves in W/NW/SW - propose W
///  if no elves in E/NE/SE - propose E
/// In the second half of the round each elf moves to their propose position:
///  if two+ elves pick the same position no elves move
/// In the next round the order of directions check is rotated, eg north becomes last.
/// Part A:
/// How many empty tiles does the rectangle contain after 10 rounds.

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

#[derive(Debug)]
struct State<'a> {
    elves: HashSet<Pos>,
    checks: Vec<(&'a str, Pos, Pos, Pos)>,
}

impl State<'_> {
    fn from(input: &str) -> Self {
        let elves = input
            .lines()
            .enumerate()
            .flat_map(|(row_idx, row)| {
                row.chars()
                    .enumerate()
                    .filter_map(move |(col_idx, content)| {
                        if content == '#' {
                            return Some(Pos {
                                row: row_idx as i64,
                                col: col_idx as i64,
                            });
                        } else {
                            None
                        }
                    })
            })
            .collect::<HashSet<_>>();

        let checks = vec![
            (
                // N
                "N",
                Pos { row: -1, col: 0 },
                Pos { row: -1, col: -1 },
                Pos { row: -1, col: 1 },
            ),
            (
                // S
                "S",
                Pos { row: 1, col: 0 },
                Pos { row: 1, col: -1 },
                Pos { row: 1, col: 1 },
            ),
            (
                // W
                "W",
                Pos { row: 0, col: -1 },
                Pos { row: -1, col: -1 },
                Pos { row: 1, col: -1 },
            ),
            (
                // E
                "E",
                Pos { row: 0, col: 1 },
                Pos { row: -1, col: 1 },
                Pos { row: 1, col: 1 },
            ),
        ];

        State { elves, checks }
    }

    fn run(&mut self, rounds: i64) -> bool {
        let mut any_moves = false;

        for _ in 0..rounds {
            let mut new_elves: HashMap<Pos, (Pos, bool)> = HashMap::new();
            for elf in &self.elves {
                let mut moved = false;

                let in_space = self.checks.iter().all(|check| {
                    !self.elves.contains(&(*elf + check.1))
                        && !self.elves.contains(&(*elf + check.2))
                        && !self.elves.contains(&(*elf + check.3))
                });

                if !in_space {
                    for check in &self.checks {
                        if !self.elves.contains(&(*elf + check.1))
                            && !self.elves.contains(&(*elf + check.2))
                            && !self.elves.contains(&(*elf + check.3))
                        {
                            let new_pos = *elf + check.1;
                            if new_elves.contains_key(&new_pos) {
                                let mut update_pos = Pos { row: 0, col: 0 };
                                if let Some((first_pos, clash)) = new_elves.get_mut(&new_pos) {
                                    *clash = true;
                                    update_pos = *first_pos;
                                }
                                new_elves.insert(update_pos.clone(), (update_pos, false));
                            } else {
                                new_elves.insert(new_pos, (*elf, false));
                                moved = true;
                            }

                            break;
                        }
                    }
                }

                if !moved {
                    new_elves.insert(elf.clone(), (*elf, false));
                }

                any_moves |= moved;
            }

            self.elves = new_elves
                .iter()
                .filter(|(_k, v)| v.1 == false)
                .map(|(k, _v)| k)
                .cloned()
                .collect::<HashSet<_>>();

            self.checks.rotate_left(1);
        }

        !any_moves
    }

    fn min_max(&self) -> (Pos, Pos) {
        let mut min = Pos {
            row: i64::MAX,
            col: i64::MAX,
        };

        let mut max = Pos {
            row: i64::MIN,
            col: i64::MIN,
        };

        for elf in &self.elves {
            min.row = min.row.min(elf.row);
            min.col = min.col.min(elf.col);

            max.row = max.row.max(elf.row);
            max.col = max.col.max(elf.col);
        }

        (min, max)
    }

    fn empty_area(&self) -> u64 {
        let (min, max) = self.min_max();
        let area = (max.row.abs_diff(min.row) + 1) * (max.col.abs_diff(min.col) + 1);

        area - self.elves.len() as u64
    }

    fn print(&self) {
        let (min, max) = self.min_max();

        for row in min.row - 1..max.row + 1 {
            for col in min.col - 1..max.col + 1 {
                if self.elves.contains(&Pos { row, col }) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }

        println!();

        for check in &self.checks {
            print!("{},", check.0)
        }

        println!("\n");
    }
}

fn main() {
    let start = time::SystemTime::now();
    let input = include_str!("../../assets/day23.txt");

    let mut state = State::from(input);
    state.print();

    let mut round = 0;
    loop {
        let is_settled = state.run(1);
        round += 1;

        if round == 10 {
            println!("Empty tiles: {}", state.empty_area());
        }

        if is_settled {
            println!("Settled at round {}", round);
            break;
        }
    }

    state.print();

    let end = time::SystemTime::now();
    println!("Took {:?}", end.duration_since(start));
}
