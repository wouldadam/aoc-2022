/// Takes a text file that includes 2 sections: a list of towers and a list of moves.
/// Applies the moves to the towers and lists the top items of each tower.
use regex::Regex;

fn main() {
    let input = include_str!("../../assets/day05.txt");

    let lines = input.split("\n\n").collect::<Vec<_>>();
    let states = lines
        .chunks(2)
        .map(|info| (parse_stacks(info[0]), parse_moves(info[1])));

    let ends = states.map(|state| run_moves(&state)).collect::<Vec<_>>();

    print!("Top items: ");
    for stack in &ends[0] {
        print!("{}", stack.last().unwrap());
    }
    println!();
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
