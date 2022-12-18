use std::collections::HashSet;

use regex::Regex;

/// Input is n sensors located on a 2d grid.
/// Each sensor reports its closes beacon. Two sensors may report the same beacon.
/// Distances use Manhattan distance.
/// Part A:
/// The target beacon has not been found by any of the sensors.
/// Count all the positions on the y=2000000 where a beacon could not be (covered by a sensor).
/// Part B:
/// A beacon is not detected by any sensor. It must have coords >= 0, <= 4000000.
/// Find its tuning frequency by multiplying its x coord by 4000000 and then adding its y coord.
/// There is only one possible position for the beacon.

type Pos = (i64, i64);

/// The grid of all sensors and beacons
struct Grid {
    sensors: HashSet<Pos>,
    beacons: HashSet<Pos>,
    areas: Vec<Area>,
    min: Pos,
    max: Pos,
}

impl Grid {
    fn has_sensor(&self, pos: &Pos) -> bool {
        self.sensors.contains(pos)
    }

    fn has_beacon(&self, pos: &Pos) -> bool {
        self.beacons.contains(pos)
    }

    fn has_item(&self, pos: &Pos) -> bool {
        self.has_sensor(pos) || self.has_beacon(pos)
    }

    fn in_area(&self, pos: &Pos) -> bool {
        self.areas.iter().any(|area| area.in_area(pos))
    }

    fn edges(&self) -> Vec<Pos> {
        self.areas
            .iter()
            .flat_map(|area| area.edges())
            .collect::<Vec<_>>()
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!("min: {:?}, max: {:?}", self.min, self.max);

        for y in self.min.1..self.max.1 {
            for x in self.min.0..self.max.0 {
                if self.has_beacon(&(x, y)) {
                    print!("b");
                } else if self.has_sensor(&(x, y)) {
                    print!("s");
                } else {
                    print!(".")
                }
            }

            println!();
        }
    }

    fn from(input: &str) -> Self {
        let line_regex = Regex::new(
            r"^Sensor at x=(-?\d*), y=(-?\d*): closest beacon is at x=(-?\d*), y=(-?\d*)$",
        )
        .unwrap();

        let mut sensors = HashSet::new();
        let mut beacons = HashSet::new();
        let mut areas = Vec::new();

        let mut min = (i64::MAX, i64::MAX);
        let mut max = (i64::MIN, i64::MIN);
        for line in input.lines() {
            let caps = line_regex.captures(line).unwrap();

            let sensor = (
                caps[1].parse::<i64>().unwrap(),
                caps[2].parse::<i64>().unwrap(),
            );
            sensors.insert(sensor);

            let beacon = (
                caps[3].parse::<i64>().unwrap(),
                caps[4].parse::<i64>().unwrap(),
            );
            beacons.insert(beacon);

            let area = Area::from(&sensor, &beacon);

            min.0 = min.0.min(area.left());
            min.1 = min.1.min(area.top());

            max.0 = max.0.max(area.bottom());
            max.1 = max.1.max(area.right());

            areas.push(area);
        }

        Grid {
            sensors,
            beacons,
            areas,
            min,
            max,
        }
    }
}

/// An area covered by a sensor
struct Area {
    pos: Pos,
    range: i64,
}

impl Area {
    fn from(sensor: &Pos, beacon: &Pos) -> Self {
        let range = m_dist(sensor, beacon);

        Area {
            pos: *sensor,
            range,
        }
    }

    fn top(&self) -> i64 {
        self.pos.1 - self.range
    }

    fn bottom(&self) -> i64 {
        self.pos.1 + self.range
    }

    fn left(&self) -> i64 {
        self.pos.0 - self.range
    }

    fn right(&self) -> i64 {
        self.pos.0 + self.range
    }

    fn in_area(&self, pos: &Pos) -> bool {
        let dist = m_dist(&self.pos, pos);

        self.range >= dist
    }

    /// Get the edges of the area as individual grid pos
    fn edges(&self) -> Vec<Pos> {
        let mut edges = Vec::new();
        let edge_dist = self.range + 1;

        for x_step in 0..edge_dist {
            let y_step = edge_dist - x_step;

            edges.push((self.pos.0 + x_step, self.pos.1 + y_step));
            edges.push((self.pos.1 + y_step, self.pos.0 - x_step));
            edges.push((self.pos.0 - x_step, self.pos.1 - y_step));
            edges.push((self.pos.1 - y_step, self.pos.0 + x_step));
        }

        edges
    }
}

/// Calculate the Manhattan distance between two positions
fn m_dist(a: &Pos, b: &Pos) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn count_known_empty(grid: &mut Grid, y: i64) -> i64 {
    let mut count = 0;

    for x in grid.min.0..grid.max.0 {
        let pos = (x, y);
        if !grid.has_item(&pos) && grid.in_area(&pos) {
            count += 1;
        }
    }

    count
}

fn find_hidden_freq(grid: &mut Grid, min: Pos, max: Pos) -> Option<(Pos, i64)> {
    for pos in grid.edges() {
        if pos.0 < min.0 || pos.1 < min.1 || pos.0 > max.0 || pos.1 > max.1 {
            continue;
        }

        if grid.has_item(&pos) || grid.in_area(&pos) {
            continue;
        }

        return Some((pos, (pos.0 * 4000000) + pos.1));
    }

    None
}

fn main() {
    let input = include_str!("../../assets/day15.txt");
    let mut grid = Grid::from(input);

    let known_empty = count_known_empty(&mut grid, 2000000);
    println!("Known empty {}", known_empty);

    let (hidden_pos, hidden_freq) =
        find_hidden_freq(&mut grid, (0, 0), (4000000, 4000000)).unwrap_or(((0, 0), -1));
    println!(
        "Hidden freq {} at {},{}",
        hidden_freq, hidden_pos.0, hidden_pos.1
    );
}
