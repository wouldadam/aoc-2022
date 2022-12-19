use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
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

struct Valve {
    label: String,
    flow_rate: i32,
    tunnels: HashSet<String>,
}

type Valves = HashMap<String, Rc<RefCell<Valve>>>;

#[derive(Debug, Clone)]
struct State {
    turned_on: HashSet<String>,
    room: String,
    previous_room: String,
    remaining: i32,
    total: i32,
    sol: String,
}

impl State {
    fn get_moves(&self, valves: &Valves) -> Vec<State> {
        let mut new_states = vec![];
        let current_valve = valves[&self.room].clone();

        // For every tunnel in the current room
        for tunnel in &current_valve.borrow().tunnels {
            // We can turn on the valve
            if !self.turned_on.contains(&self.room) && current_valve.borrow().flow_rate > 0 {
                // We can turn on and move
                if self.remaining >= 2 {
                    let mut new_state = self.clone();
                    new_state.turned_on.insert(self.room.clone());
                    new_state.total += (new_state.remaining - 1) * current_valve.borrow().flow_rate;
                    new_state.remaining -= 2;
                    new_state.previous_room = self.room.clone();
                    new_state.room = tunnel.clone();
                    new_state.sol = format!("{} O -> {}", new_state.sol, tunnel);

                    new_states.push(new_state);
                } else if self.remaining == 1 {
                    // Or just turn on if at the end
                    let mut new_state = self.clone();
                    new_state.turned_on.insert(self.room.clone());
                    new_state.total += (new_state.remaining - 1) * current_valve.borrow().flow_rate;
                    new_state.remaining -= 1;
                    new_state.previous_room = self.room.clone();
                    new_state.sol = format!("{} O", new_state.sol);

                    new_states.push(new_state);
                }
            }

            if *tunnel == self.previous_room {
                continue;
            }

            // We can move to it without turning the valve, no point unless we we be able to do something
            if self.remaining >= 2 {
                let mut new_state = self.clone();
                new_state.remaining -= 1;
                new_state.previous_room = new_state.room;
                new_state.room = tunnel.clone();
                new_state.sol = format!("{} -> {}", new_state.sol, tunnel);

                new_states.push(new_state);
            }
        }

        new_states
    }
}

impl Valve {
    fn from(input: &str) -> Valves {
        let reg = Regex::new(r"^Valve ([[:upper:]]{2}) has flow rate=(\d*); tunnel[s]? lead[s]? to valve[s]? ((?:[[:upper:]]{2}(?:, *)?)*)$").unwrap();
        let mut valves = Valves::new();

        // Add all valves
        for line in input.lines() {
            let caps = reg.captures(line).unwrap();
            let valve = Rc::new(RefCell::new(Valve {
                label: String::from(&caps[1]),
                flow_rate: caps[2].parse::<i32>().unwrap(),
                tunnels: HashSet::new(),
            }));

            valves.insert(valve.borrow().label.clone(), valve.clone());
        }

        // Add the tunnels
        for line in input.lines() {
            let caps = reg.captures(line).unwrap();
            let label = String::from(&caps[1]);

            for tunnel_to in caps[3].split(',').map(|l| l.trim()) {
                valves[&label]
                    .borrow_mut()
                    .tunnels
                    .insert(String::from(tunnel_to));
            }
        }

        valves
    }
}

fn main() {
    let input = include_str!("../../assets/day16.txt");
    let valves = Valve::from(input);

    let mut best = State {
        turned_on: HashSet::new(),
        room: String::from("AA"),
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
}
