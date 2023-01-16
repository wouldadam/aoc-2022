use std::{collections::HashMap, time};

use regex::Regex;

type Op = Box<dyn Fn(i64, i64) -> i64>;
struct Monkey {
    id: String,
    inputs: (String, String),
    output: Op,
}

fn load(input: &str) -> (Vec<Monkey>, HashMap<String, i64>) {
    let op_re = Regex::new(r"^(.*): (.*) ([\+\-/\*]) (.*)$").unwrap();
    let number_re = Regex::new(r"^(.*): (\d*)$").unwrap();

    let mut answers: HashMap<String, i64> = HashMap::new();
    let mut monkeys = Vec::new();

    for line in input.lines() {
        if let Some(captures) = op_re.captures(line) {
            let op: Op = match &captures[3] {
                "+" => Box::new(|a, b| a + b),
                "-" => Box::new(|a, b| a - b),
                "/" => Box::new(|a, b| a / b),
                "*" => Box::new(|a, b| a * b),
                _ => panic!("Invalid op"),
            };

            monkeys.push(Monkey {
                id: captures[1].to_string(),
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

fn main() {
    let start = time::SystemTime::now();

    let input = include_str!("../../assets/day21.txt");
    let (monkeys, mut answers) = load(input);

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

    let end = time::SystemTime::now();
    println!("Took {:?}", end.duration_since(start));
}
