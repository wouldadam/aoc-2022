/// Input contains a list of monkeys.
/// Every round each monkey, in turn, inspects their current item performing the stated operation.
/// Th item value is then divided by three.
/// A test is then performed on the item to decide which other monkey to throw the item to.
/// Part A:
/// Monkey business is the two larges monkey inspection counts multiplied together.
/// Calculate the monkey business.

struct Monkey {
    items: Vec<i32>,
    op: Box<dyn Fn(i32) -> i32>,
    test: Box<dyn Fn(i32) -> usize>,
}

fn load(input: &str) -> Vec<Monkey> {
    let monkeys = input
        .split("\n\n")
        .map(|monkey| monkey.lines().map(String::from).collect::<Vec<_>>())
        .map(|monkey| {
            // Items
            let items = monkey[1]
                .split(':')
                .last()
                .unwrap()
                .split(',')
                .map(|item| item.trim().parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            // Operation
            let op: Box<dyn Fn(i32) -> i32>;
            {
                let op_parts = monkey[2]
                    .split("new = old")
                    .last()
                    .unwrap()
                    .trim()
                    .split(' ')
                    .map(String::from)
                    .collect::<Vec<_>>();

                op = match (op_parts[0].as_str(), op_parts[1].as_str()) {
                    ("*", "old") => Box::new(move |old: i32| old * old),
                    ("+", "old") => Box::new(move |old: i32| old + old),
                    ("*", _val) => {
                        Box::new(move |old: i32| old * op_parts[1].parse::<i32>().unwrap())
                    }
                    ("+", _val) => {
                        Box::new(move |old: i32| old + op_parts[1].parse::<i32>().unwrap())
                    }
                    _ => unreachable!("Invalid operation"),
                };
            }

            // Test
            let true_monkey = monkey[4]
                .split(' ')
                .last()
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap();
            let false_monkey = monkey[5]
                .split(' ')
                .last()
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap();
            let div = monkey[3]
                .split("divisible by ")
                .last()
                .unwrap()
                .trim()
                .parse::<i32>()
                .unwrap();

            let test = Box::new(move |val: i32| {
                if val % div == 0 {
                    return true_monkey;
                }

                false_monkey
            });

            Monkey { items, op, test }
        })
        .collect::<Vec<_>>();

    monkeys
}

fn calc_monkey_business(mut monkeys: Vec<Monkey>, rounds: usize) {
    let mut inspections = vec![0; monkeys.len()];

    for _round in 0..rounds {
        for monkey_idx in 0..monkeys.len() {
            let items;
            {
                let monkey = &mut monkeys[monkey_idx];

                for item in &mut monkey.items {
                    {
                        // Inspect
                        *item = monkey.op.as_ref()(*item);
                        inspections[monkey_idx] += 1;

                        // Worry
                        *item /= 3;
                    }
                }

                items = monkey.items.clone();
                monkey.items = vec![];
            }

            for item in items {
                let monkey = &mut monkeys[monkey_idx];
                let new_monkey_idx = monkey.test.as_ref()(item);
                let new_monkey = &mut monkeys[new_monkey_idx];
                new_monkey.items.push(item);
            }
        }
    }

    inspections.sort_unstable();
    inspections.reverse();
    dbg!(&inspections);

    let monkey_business = inspections[0] * inspections[1];
    println!("Monkey business: {}", monkey_business);
}

fn main() {
    let input = include_str!("../../assets/day11.txt");
    let monkeys = load(input);

    calc_monkey_business(monkeys, 20);
}
