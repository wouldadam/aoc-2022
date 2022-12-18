use std::collections::HashMap;

use regex::Regex;

/// Input is n sensors located on a 2d grid.
/// Each sensor reports its closes beacon. Two sensors may report the same beacon.
/// Distances use Manhattan distance.
/// Part A:
/// The target beacon has not been found by any of the sensors.
/// Count all the positions on the y=2000000 where a beacon could not be (covered by a sensor).

type Pos = (i64, i64);
type ReadingVec = Vec<Reading>;

#[derive(Clone, Copy)]
struct Reading {
    sensor: Pos,
    beacon: Pos,
    sb_dist: i64,
}

#[derive(Clone, Copy)]
enum Cell {
    Empty,
    Sensor(Reading),
    Beacon,
}

struct Grid {
    data: HashMap<Pos, Cell>,
    min: Pos,
    max: Pos,
}

impl Grid {
    fn new(min: Pos, max: Pos) -> Self {
        Grid {
            data: HashMap::new(),
            min,
            max,
        }
    }

    fn at(&self, pos: Pos) -> Cell {
        if self.data.contains_key(&pos) {
            return self.data[&pos];
        }

        Cell::Empty
    }

    fn set(&mut self, pos: Pos, val: Cell) {
        self.data.insert(pos, val);
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!("min: {:?}, max: {:?}", self.min, self.max);

        for y in self.min.1..self.max.1 {
            for x in self.min.0..self.max.0 {
                let empty = Cell::Empty;
                let cell = self.data.get(&(x, y)).unwrap_or(&empty);
                match cell {
                    Cell::Empty => print!("."),
                    Cell::Sensor(_) => print!("s"),
                    Cell::Beacon => print!("b"),
                }
            }

            println!();
        }
    }
}

/// Calculate the Manhattan distance between two positions
fn m_dist(a: Pos, b: Pos) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn load_readings(input: &str) -> (ReadingVec, Pos, Pos) {
    let line_regex =
        Regex::new(r"^Sensor at x=(-?\d*), y=(-?\d*): closest beacon is at x=(-?\d*), y=(-?\d*)$")
            .unwrap();

    let mut readings = vec![];
    let mut min = (i64::MAX, i64::MAX);
    let mut max = (i64::MIN, i64::MIN);
    for line in input.lines() {
        let caps = line_regex.captures(line).unwrap();

        let sensor = (
            caps[1].parse::<i64>().unwrap(),
            caps[2].parse::<i64>().unwrap(),
        );
        let beacon = (
            caps[3].parse::<i64>().unwrap(),
            caps[4].parse::<i64>().unwrap(),
        );

        let sb_dist = m_dist(sensor, beacon);

        min.0 = min.0.min((sensor.0 - sb_dist).min(beacon.0));
        min.1 = min.1.min((sensor.1 - sb_dist).min(beacon.1));

        max.0 = max.0.max((sensor.0 + sb_dist).max(beacon.0));
        max.1 = max.1.max((sensor.1 + sb_dist).max(beacon.1));

        readings.push(Reading {
            sensor,
            beacon,
            sb_dist,
        });
    }

    (readings, min, max)
}

fn build_grid(readings: &ReadingVec, min: Pos, max: Pos) -> Grid {
    let mut grid = Grid::new(min, max);

    for reading in readings {
        grid.set(reading.sensor, Cell::Sensor(*reading));
        grid.set(reading.beacon, Cell::Beacon);
    }

    grid
}

fn count_known_empty(grid: &mut Grid, sensors: &ReadingVec, min: Pos, max: Pos, y: i64) -> i64 {
    let mut count = 0;

    for x in min.0..max.0 {
        for reading in sensors {
            match grid.at((x, y)) {
                Cell::Empty => {
                    let sp_dist = m_dist(reading.sensor, (x, y));
                    if sp_dist <= reading.sb_dist {
                        count += 1;
                        break;
                    }
                }
                Cell::Sensor(_) | Cell::Beacon => {}
            }
        }
    }

    count
}

fn main() {
    let input = include_str!("../../assets/day15.txt");
    let (sensors, min, max) = load_readings(input);
    let mut grid = build_grid(&sensors, min, max);

    let checked = count_known_empty(&mut grid, &sensors, min, max, 2000000);

    println!("Checked {}", checked);
}
