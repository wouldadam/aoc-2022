use rayon::prelude::*;
use regex::Regex;
use std::{
    collections::{BTreeSet, HashMap},
    time,
};

/// The input describes a network of pipes and pressure release valves in an angry volcano.
/// The valves are in different rooms.
/// Each valve has a label, flow rate (when open, per minute) and tunnels between the rooms/valves.
/// It takes 1 minute to open a valve.
/// It takes 1 minute to move between tunnels.
/// You are in the room with the valve labeled AA.
/// Part A:
/// What is the most pressure that can be released?
/// Part B:
/// What if an elephant helps? It takes 4 minutes to teach them what to do.

#[derive(Clone)]
struct Valve {
    label: String,
    flow_rate: i32,
    tunnels: Vec<String>,
    costs: HashMap<String, i32>,
}

type Valves = HashMap<String, Valve>;

#[derive(Debug, Clone)]
struct State {
    turned_on: BTreeSet<String>,
    total: i32,
    plumbers: Vec<Plumber>,
}

#[derive(Debug, Clone)]
struct Plumber {
    current_room: String,
    previous_room: String,
    remaining: i32,
    path: String,
}

impl State {
    fn get_moves(&self, valves: &Valves, plumber_idx: usize) -> Vec<State> {
        let mut moves = vec![];
        let valve = &valves[&self.plumbers[plumber_idx].current_room];

        // Each move moves to a dest and turns on the valve.
        // We always turn the valve on as out dests cover paths to all valves.
        for (dest, cost) in &valve.costs {
            if !self.turned_on.contains(dest)
                && self.plumbers[plumber_idx].remaining > *cost
                && valves[dest].flow_rate > 0
            {
                let mut new_state = self.clone();
                new_state.turned_on.insert(dest.clone());
                new_state.plumbers[plumber_idx].remaining -= cost + 1;
                new_state.total +=
                    new_state.plumbers[plumber_idx].remaining * valves[dest].flow_rate;
                new_state.plumbers[plumber_idx].previous_room =
                    new_state.plumbers[plumber_idx].current_room.clone();
                new_state.plumbers[plumber_idx].current_room = dest.clone();
                new_state.plumbers[plumber_idx].path =
                    format!("{} -> {}", new_state.plumbers[plumber_idx].path, dest);

                moves.push(new_state);
            }
        }

        moves
    }
}

impl Valve {
    fn from(input: &str) -> Valves {
        let reg = Regex::new(r"^Valve ([[:upper:]]{2}) has flow rate=(\d*); tunnel[s]? lead[s]? to valve[s]? ((?:[[:upper:]]{2}(?:, *)?)*)$").unwrap();

        // Parse the input
        let mut valves = input
            .par_lines()
            .map(|line| {
                let caps = reg.captures(line).unwrap();
                let tunnels = caps[3]
                    .split(',')
                    .map(|l| l.trim().to_string())
                    .collect::<Vec<_>>();

                let valve = Valve {
                    label: String::from(&caps[1]),
                    flow_rate: caps[2].parse::<i32>().unwrap(),
                    tunnels,
                    costs: HashMap::new(),
                };

                (valve.label.clone(), valve)
            })
            .collect::<HashMap<String, Valve>>();

        // Work out the cost for moving between any two valves
        let total_valves = valves.len();
        let valve_costs = valves
            .par_iter()
            .map(|(label, valve)| {
                let mut costs = HashMap::new();
                let mut cost = 1;

                let mut layer = valve.tunnels.clone();
                while costs.len() < total_valves - 1 {
                    let mut next_layer: Vec<String> = vec![];

                    for dest in &layer {
                        if !costs.contains_key(dest) && *dest != valve.label {
                            costs.insert(dest.clone(), cost);

                            next_layer.extend(valves[dest].tunnels.clone());
                        }
                    }

                    layer = next_layer;
                    cost += 1;
                }

                (label.clone(), costs)
            })
            .collect::<HashMap<String, HashMap<String, i32>>>();

        for (label, costs) in valve_costs {
            valves.get_mut(&label).unwrap().costs = costs;
        }

        valves
    }
}

fn find_best(valves: &HashMap<String, Valve>) {
    let mut best = State {
        turned_on: BTreeSet::new(),
        total: 0,
        plumbers: vec![Plumber {
            current_room: String::from("AA"),
            previous_room: String::from(""),
            remaining: 30,
            path: String::from("AA"),
        }],
    };

    let mut stack = vec![best.clone()];
    while let Some(next) = stack.pop() {
        let moves = next.get_moves(valves, 0);

        for mv in moves {
            if mv.plumbers[0].remaining > 0 {
                stack.push(mv.clone());
            }

            if mv.total > best.total {
                best = mv;
                println!("Best solution: {:?}", best);
            }
        }
    }
}

#[allow(dead_code)]
fn find_best_with_elephant_slow(valves: &HashMap<String, Valve>) {
    let mut best = State {
        turned_on: BTreeSet::new(),
        total: 0,
        plumbers: vec![
            Plumber {
                current_room: String::from("AA"),
                previous_room: String::from(""),
                remaining: 26,
                path: String::from("AA"),
            },
            Plumber {
                current_room: String::from("AA"),
                previous_room: String::from(""),
                remaining: 26,
                path: String::from("AA"),
            },
        ],
    };

    let mut stack = vec![best.clone()];
    while let Some(next) = stack.pop() {
        let plumber_idx = if next.plumbers[0].remaining > next.plumbers[1].remaining {
            0
        } else {
            1
        };

        let moves = next.get_moves(valves, plumber_idx);

        for mv in moves {
            if mv.plumbers[0].remaining > 0 {
                stack.push(mv.clone());
            }

            if mv.total > best.total {
                best = mv;
                println!("Best solution: {:?}", best);
            }
        }
    }
}

fn find_best_with_elephant(valves: &HashMap<String, Valve>) {
    // Find all possible single path solutions
    let start = State {
        turned_on: BTreeSet::new(),
        total: 0,
        plumbers: vec![Plumber {
            current_room: String::from("AA"),
            previous_room: String::from(""),
            remaining: 26,
            path: String::from("AA"),
        }],
    };

    let mut best_paths: HashMap<BTreeSet<String>, State> = HashMap::new();
    let mut stack = vec![start];
    while let Some(next) = stack.pop() {
        let moves = next.get_moves(valves, 0);

        for mv in moves {
            if mv.plumbers[0].remaining > 0 {
                stack.push(mv.clone());
            }

            if !best_paths.contains_key(&mv.turned_on) || best_paths[&mv.turned_on].total < mv.total
            {
                best_paths.insert(mv.turned_on.clone(), mv.clone());
            }
        }
    }

    // Find the best combination paths that don't use the same valves
    let mut best = 0;
    for (path_a, state_a) in &best_paths {
        for (path_b, state_b) in &best_paths {
            if path_a.is_disjoint(path_b) {
                let total = state_a.total + state_b.total;
                best = best.max(total);
            }
        }
    }

    println!("Best total with 2: {}", best);
}

fn main() {
    let start = time::SystemTime::now();

    let input = include_str!("../../assets/day16.txt");
    let valves = Valve::from(input);

    find_best(&valves);

    println!();

    find_best_with_elephant(&valves);

    let end = time::SystemTime::now();
    println!("Took {:?}", end.duration_since(start));
}
