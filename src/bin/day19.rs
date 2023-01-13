use std::{
    collections::HashSet,
    ops::{Add, AddAssign, Sub, SubAssign},
    time,
};

use regex::Regex;

/// Input contains blueprints for different robot factory configs.
/// It takes a minute for the factory to build a robot.
/// It takes a minute for a robot to collect 1 of its target resource.
/// You start with 1 ore robot.
/// Part A:
/// The quality of a blueprint is id * number of geodes in 24 minutes.
/// Sum the quality of all blueprints.

const DURATION_MINS: i32 = 24;
const ACTION_MINS: i32 = 1;

// (ore, clay, obsidian, geode)
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Materials(i32, i32, i32, i32);

impl Add for Materials {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Materials(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3 + rhs.3,
        )
    }
}

impl AddAssign for Materials {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
        self.3 += rhs.3;
    }
}

impl Sub for Materials {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Materials(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
            self.3 - rhs.3,
        )
    }
}

impl SubAssign for Materials {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
        self.3 -= rhs.3;
    }
}

impl Materials {
    fn can_afford(&self, price: Materials) -> bool {
        let res = *self - price;

        res.0 >= 0 && res.1 >= 0 && res.2 >= 0 && res.3 >= 0
    }

    fn max(&self, rhs: Materials) -> Self {
        Materials(
            self.0.max(rhs.0),
            self.1.max(rhs.1),
            self.2.max(rhs.2),
            self.2.max(rhs.2),
        )
    }
}

struct Blueprint {
    id: i32,
    ore_robot_cost: Materials,
    clay_robot_cost: Materials,
    obsidian_robot_cost: Materials,
    geode_robot_cost: Materials,
}

impl Blueprint {
    fn from(line: &str) -> Self {
        let reg = Regex::new(r"^Blueprint (\d*): Each ore robot costs (\d*) ore. Each clay robot costs (\d*) ore. Each obsidian robot costs (\d*) ore and (\d*) clay. Each geode robot costs (\d*) ore and (\d*) obsidian.$").unwrap();
        let caps = reg.captures(line).unwrap();

        Blueprint {
            id: caps[1].parse::<i32>().unwrap(),
            ore_robot_cost: Materials(caps[2].parse::<i32>().unwrap(), 0, 0, 0),
            clay_robot_cost: Materials(caps[3].parse::<i32>().unwrap(), 0, 0, 0),
            obsidian_robot_cost: Materials(
                caps[4].parse::<i32>().unwrap(),
                caps[5].parse::<i32>().unwrap(),
                0,
                0,
            ),
            geode_robot_cost: Materials(
                caps[6].parse::<i32>().unwrap(),
                0,
                caps[7].parse::<i32>().unwrap(),
                0,
            ),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State {
    remaining_mins: i32,
    storage: Materials,
    robots: Materials,
    next_robot: Option<Materials>,
}

impl State {
    fn buy(&self, robot: Materials, cost: Materials) -> Option<Self> {
        if self.storage.can_afford(cost) {
            let mut n = self.clone();
            n.storage -= cost;
            n.next_robot = Some(robot);
            return Some(n);
        }

        None
    }
}

const ORE_ROBOT: Materials = Materials(1, 0, 0, 0);
const CLAY_ROBOT: Materials = Materials(0, 1, 0, 0);
const OBSIDIAN_ROBOT: Materials = Materials(0, 0, 1, 0);
const GEODE_ROBOT: Materials = Materials(0, 0, 0, 1);

fn sum_qualities(blueprints: &[Blueprint]) -> i32 {
    let sum: i32 = blueprints.iter().map(calc_quality).sum();
    sum
}

/// If can afford everything then don't do nothing
/// Don't build more robots than needed to build another robot
/// Ignore state that you have already reached
fn calc_quality(blueprint: &Blueprint) -> i32 {
    let start_time = time::SystemTime::now();
    println!("Starting blueprint {}", blueprint.id);

    // The max you might need to spend on a material to buy a robot.
    // We will only every but this much of each robot.
    let max_costs = blueprint
        .ore_robot_cost
        .max(blueprint.clay_robot_cost)
        .max(blueprint.obsidian_robot_cost)
        .max(blueprint.geode_robot_cost);

    let start = State {
        remaining_mins: DURATION_MINS,
        storage: Materials(0, 0, 0, 0),
        robots: Materials(1, 0, 0, 0),
        next_robot: None,
    };

    let mut seen = HashSet::new();

    let mut best = 0;
    let mut states = vec![start];

    while let Some(mut state) = states.pop() {
        // Have all robots do their work
        state.storage += state.robots;
        state.remaining_mins -= ACTION_MINS;

        if state.remaining_mins <= 0 {
            best = best.max(blueprint.id * state.storage.3);
            continue;
        }

        // Take delivery of any new robot
        if let Some(robot) = state.next_robot {
            state.robots += robot;
            state.next_robot = None;
        }

        // If we have got to this state before abandon this branch
        if seen.contains(&state) {
            continue;
        }

        seen.insert(state.clone());

        // Enumerate our possible moves
        let mut afford = 0;

        if let Some(s) = state.buy(GEODE_ROBOT, blueprint.geode_robot_cost) {
            states.push(s);
            afford += 1;
        }

        if state.robots.2 < max_costs.2 {
            if let Some(s) = state.buy(OBSIDIAN_ROBOT, blueprint.obsidian_robot_cost) {
                states.push(s);
                afford += 1;
            }
        }

        if state.robots.1 < max_costs.1 {
            if let Some(s) = state.buy(CLAY_ROBOT, blueprint.clay_robot_cost) {
                states.push(s);
                afford += 1;
            }
        }

        if state.robots.0 < max_costs.0 {
            if let Some(s) = state.buy(ORE_ROBOT, blueprint.ore_robot_cost) {
                states.push(s);
                afford += 1;
            }
        }

        if afford != 4 {
            states.push(state.clone());
        }
    }

    let end_time = time::SystemTime::now();

    println!(
        "Found best: {}, took {:?}",
        best,
        end_time.duration_since(start_time)
    );
    best
}

fn main() {
    let start = time::SystemTime::now();

    let input = include_str!("../../assets/day19.txt");
    let blueprints = input.lines().map(Blueprint::from).collect::<Vec<_>>();

    let sum = sum_qualities(&blueprints);
    println!("Quality sum: {}", sum);

    let end = time::SystemTime::now();
    println!("Took {:?}", end.duration_since(start));
}
