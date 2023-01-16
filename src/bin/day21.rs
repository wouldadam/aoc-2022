use std::{collections::HashMap, time};

use regex::Regex;

/// Input is a list of monkeys that either know a number or an operation, first val is their id.
/// Part A:
/// Find the result of the operation for the monkey with id "root".
/// Part B:
/// The monkey with id "root" is now an equality check.
/// The monkey with id "humn" is actually you, its number is not relevant.
/// Find the number for "humn" so that the "root" equality check passes.

type Op = Box<dyn Fn(i64, i64) -> i64>;

struct Monkey {
    id: String,
    inputs: (String, String),
    output: Op,
}

fn load(input: &str, is_root_comp: bool) -> (Vec<Monkey>, HashMap<String, i64>) {
    let op_re = Regex::new(r"^(.*): (.*) ([\+\-/\*]) (.*)$").unwrap();
    let number_re = Regex::new(r"^(.*): (\d*)$").unwrap();

    let mut answers: HashMap<String, i64> = HashMap::new();
    let mut monkeys = Vec::new();

    for line in input.lines() {
        if let Some(captures) = op_re.captures(line) {
            let id = captures[1].to_string();

            let op: Op = if id == "root" && is_root_comp {
                Box::new(|a, b| a - b)
            } else {
                match &captures[3] {
                    "+" => Box::new(|a, b| a + b),
                    "-" => Box::new(|a, b| a - b),
                    "/" => Box::new(|a, b| a / b),
                    "*" => Box::new(|a, b| a * b),
                    _ => panic!("Invalid op"),
                }
            };

            monkeys.push(Monkey {
                id,
                inputs: (captures[2].to_string(), captures[4].to_string()),
                output: op,
            });
        } else if let Some(captures) = number_re.captures(line) {
            answers.insert(captures[1].to_string(), captures[2].parse::<i64>().unwrap());
        } else {
            panic!("Invalid input: {}", line);
        }
    }

    (monkeys, answers)
}

fn find_root_val(monkeys: Vec<Monkey>, mut answers: HashMap<String, i64>) {
    loop {
        for monkey in &monkeys {
            if !answers.contains_key(&monkey.id) {
                let ans_a = answers.get(&monkey.inputs.0);
                let ans_b = answers.get(&monkey.inputs.1);
                if let (Some(a), Some(b)) = (ans_a, ans_b) {
                    answers.insert(monkey.id.clone(), monkey.output.as_ref()(*a, *b));
                }
            }
        }

        if let Some(val) = answers.get("root") {
            println!("root = {}", val);
            break;
        }
    }
}

fn find_humn_val(monkeys: Vec<Monkey>, starting_answers: HashMap<String, i64>) {
    let mut val = 3111001100000;

    loop {
        let mut answers = starting_answers.clone();
        answers.insert(String::from("humn"), val);

        loop {
            for monkey in &monkeys {
                if !answers.contains_key(&monkey.id) {
                    let ans_a = answers.get(&monkey.inputs.0);
                    let ans_b = answers.get(&monkey.inputs.1);
                    if let (Some(a), Some(b)) = (ans_a, ans_b) {
                        answers.insert(monkey.id.clone(), monkey.output.as_ref()(*a, *b));
                    }
                }
            }

            if let Some(root_val) = answers.get("root") {
                match root_val.cmp(&0) {
                    std::cmp::Ordering::Less => {
                        val += root_val / 10;
                        break;
                    }
                    std::cmp::Ordering::Greater => {
                        val += root_val / 10;
                        break;
                    }
                    std::cmp::Ordering::Equal => {
                        println!("humn = {}", val);
                        return;
                    }
                }
            }
        }
    }
}

fn main() {
    let start = time::SystemTime::now();
    let input = include_str!("../../assets/day21.txt");

    // Part a
    {
        let (monkeys, answers) = load(input, false);
        find_root_val(monkeys, answers);
    }

    // Part b
    {
        let (monkeys, answers) = load(input, true);
        find_humn_val(monkeys, answers);
    }

    let end = time::SystemTime::now();
    println!("Took {:?}", end.duration_since(start));
}
