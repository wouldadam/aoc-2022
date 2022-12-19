use std::{
    collections::{HashMap, HashSet},
    time,
};

use regex::Regex;

/// The input describes a network of pipes and pressure release valves in an angry volcano.
/// The valves are in different rooms.
/// Each valve has a label, flow rate (when open, per minute) and tunnels between the rooms/valves.
/// It takes 1 minute to open a valve.
/// It takes 1 minute to move between tunnels.
/// You are in the room with the valve labeled AA.
/// Part A:
/// What is the most pressure that can be released?

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
    turned_on: HashSet<String>,
    current_room: String,
    previous_room: String,
    remaining: i32,
    total: i32,
    sol: String,
}

impl State {
    fn get_moves(&self, valves: &Valves) -> Vec<State> {
        let mut moves = vec![];
        let valve = &valves[&self.current_room];

        // Each move moves to a dest and turns on the valve.
        // We always turn the valve on as out dests cover paths to all valves.
        for (dest, cost) in &valve.costs {
            if !self.turned_on.contains(dest)
                && self.remaining > *cost
                && valves[dest].flow_rate > 0
            {
                let mut new_state = self.clone();
                new_state.turned_on.insert(dest.clone());
                new_state.remaining -= cost + 1;
                new_state.total += new_state.remaining * valves[dest].flow_rate;
                new_state.previous_room = new_state.current_room;
                new_state.current_room = dest.clone();
                new_state.sol = format!("{} -> {}", new_state.sol, dest);

                moves.push(new_state);
            }
        }

        moves
    }
}

impl Valve {
    fn from(input: &str) -> Valves {
        let reg = Regex::new(r"^Valve ([[:upper:]]{2}) has flow rate=(\d*); tunnel[s]? lead[s]? to valve[s]? ((?:[[:upper:]]{2}(?:, *)?)*)$").unwrap();
        let mut valves = Valves::new();

        // Parse the input
        for line in input.lines() {
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

            valves.insert(valve.label.clone(), valve);
        }

        // Work out the cost for moving between any two valves
        let total_valves = valves.len();
        let mut valve_costs = HashMap::new();
        for valve in valves.values() {
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

            valve_costs.insert(valve.label.clone(), costs);
        }

        for (label, costs) in valve_costs {
            valves.get_mut(&label).unwrap().costs = costs;
        }

        valves
    }
}

fn main() {
    let start = time::SystemTime::now();
    let input = include_str!("../../assets/day16.txt");
    let valves = Valve::from(input);

    let mut best = State {
        turned_on: HashSet::new(),
        current_room: String::from("AA"),
        previous_room: String::from(""),
        remaining: 30,
        total: 0,
        sol: String::from("AA"),
    };

    let mut stack = vec![best.clone()];
    while let Some(next) = stack.pop() {
        let moves = next.get_moves(&valves);

        for mv in moves {
            if mv.remaining > 0 {
                stack.push(mv.clone());
            }

            if mv.total > best.total {
                best = mv;
                println!("Best solution: {:?}", best);
            }
        }
    }

    let end = time::SystemTime::now();
    println!("Took {:?}", end.duration_since(start));
}
