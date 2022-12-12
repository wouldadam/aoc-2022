/// Input contains a list of monkeys.
/// Every round each monkey, in turn, inspects their current item performing the stated operation.
/// A test is then performed on the item to decide which other monkey to throw the item to.
/// Monkey business is the two largest monkey inspection counts multiplied together.
/// Part A:
/// The item value is divided by three between the inspection and test.
/// Calculate the monkey business after 20 rounds.
/// Part B.
/// Calculate the monkey business after 10000 rounds.

struct Monkey {
    items: Vec<u128>,
    op: Box<dyn Fn(u128) -> u128>,
    test: Box<dyn Fn(u128) -> usize>,
}

fn load(input: &str) -> (Vec<Monkey>, u128) {
    let mut lcm = 1;
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
                .map(|item| item.trim().parse::<u128>().unwrap())
                .collect::<Vec<_>>();

            // Operation
            let op: Box<dyn Fn(u128) -> u128>;
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
                    ("*", "old") => Box::new(move |old: u128| old * old),
                    ("+", "old") => Box::new(move |old: u128| old + old),
                    ("*", _val) => {
                        Box::new(move |old: u128| old * op_parts[1].parse::<u128>().unwrap())
                    }
                    ("+", _val) => {
                        Box::new(move |old: u128| old + op_parts[1].parse::<u128>().unwrap())
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
                .parse::<u128>()
                .unwrap();

            lcm *= div;
            let test = Box::new(move |val: u128| {
                if val % div == 0 {
                    return true_monkey;
                }

                false_monkey
            });

            Monkey { items, op, test }
        })
        .collect::<Vec<_>>();

    (monkeys, lcm)
}

fn calc_monkey_business(mut monkeys: Vec<Monkey>, rounds: usize, div: u128, lcm: u128) {
    let mut inspections: Vec<u128> = vec![0; monkeys.len()];

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
                        *item /= div;

                        *item %= lcm;
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

    let part_a = load(input);
    calc_monkey_business(part_a.0, 20, 3, part_a.1);

    let part_b = load(input);
    calc_monkey_business(part_b.0, 10000, 1, part_b.1);
}
