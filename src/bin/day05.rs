/// Takes a text file that includes 2 sections: a list of towers and a list of moves.
/// Part A:
/// Applies the moves to the towers and lists the top items of each tower.
/// Part B:
/// Moving multiple crates now maintains the order.
use regex::Regex;

fn main() {
    let input = include_str!("../../assets/day05.txt");

    let lines = input.split("\n\n").collect::<Vec<_>>();
    let start_state = lines
        .chunks(2)
        .map(|info| (parse_stacks(info[0]), parse_moves(info[1])))
        .last()
        .unwrap();

    let part_a_end = run_moves(&start_state);
    let part_a_top = part_a_end
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>();

    let part_b_end = run_multi_moves(&start_state);
    let part_b_top = part_b_end
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>();

    println!("Part A top items: {}", part_a_top);
    println!("Part B top items: {}", part_b_top);
}

fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = vec![];

    input.lines().rev().for_each(|line| {
        let chars = line.chars().collect::<Vec<_>>();
        let stack_count = f32::ceil(chars.len() as f32 / 4.0f32) as usize;
        for stack_idx in 0..stack_count {
            let char_idx = 1 + (stack_idx * 4);
            if chars[char_idx].is_alphabetic() {
                if stacks.len() <= stack_idx {
                    stacks.push(vec![]);
                }

                stacks[stack_idx].push(chars[char_idx]);
            }
        }
    });

    stacks
}
#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
    amount: u32,
}

fn parse_moves(input: &str) -> Vec<Move> {
    let reg = Regex::new(r"^move (\d*) from (\d*) to (\d*)$").unwrap();

    input
        .lines()
        .map(|line| reg.captures(line).unwrap())
        .map(|capture| Move {
            amount: capture[1].parse().unwrap(),
            from: capture[2].parse::<usize>().unwrap() - 1,
            to: capture[3].parse::<usize>().unwrap() - 1,
        })
        .collect()
}

fn run_moves(state: &(Vec<Vec<char>>, Vec<Move>)) -> Vec<Vec<char>> {
    let mut stacks = state.0.clone();

    for mv in &state.1 {
        for _ in 0..mv.amount {
            let c = stacks[mv.from].pop().unwrap();
            stacks[mv.to].push(c);
        }
    }

    stacks
}

fn run_multi_moves(state: &(Vec<Vec<char>>, Vec<Move>)) -> Vec<Vec<char>> {
    let mut stacks = state.0.clone();

    for mv in &state.1 {
        let mut items = vec![];
        for _ in 0..mv.amount {
            items.push(stacks[mv.from].pop().unwrap());
        }

        items.reverse();
        for c in items {
            stacks[mv.to].push(c);
        }
    }

    stacks
}
