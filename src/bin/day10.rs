/// Input is a series of operations.
/// noop takes one cycle
/// addx adds to the register x (starts a 1), takes 2 cycles.
/// Part A:
/// Strength at a given cycle is cycle * register
/// Calculate the sum of the strengths at cycles 20, 60, 100, 140, 180, 220
/// Part B:
/// A screen is 40 pixels wide and 6 deep. It is rendered rtl ttb, once pixel per cycle.
/// The horizontal center ps of the item being rendered is in the register and it is 3 pixels wide.
/// Render the screen and find the drawn letters.

struct Op {
    add: i32,
    end_cycle: i32,
}

fn load_ops(input: &str) -> Vec<Op> {
    let mut ops = vec![];

    let mut cycle = 0;
    for line in input.lines() {
        if line.starts_with("noop") {
            ops.push(Op {
                add: 0,
                end_cycle: cycle,
            });
            cycle += 1;
        } else {
            ops.push(Op {
                add: line.split(' ').last().unwrap().parse::<i32>().unwrap(),
                end_cycle: cycle + 1,
            });
            cycle += 2;
        }
    }

    ops
}

fn sum_cycles_of_interest(ops: &[Op]) {
    let mut reg = 1;
    let mut sum = 0;

    let mut interest = vec![20, 60, 100, 140, 180, 220];
    interest.reverse();

    for op in ops {
        let next = *interest.last().unwrap_or(&i32::MAX);

        if op.end_cycle >= next {
            sum += next * reg;
            interest.pop();
        }

        reg += op.add;
    }

    println!("At end reg {}, sum {}", reg, sum);
}

fn render_screen(ops: &[Op]) {
    let mut op_idx = 0;
    let mut reg = 1;

    for cycle in 0..240 {
        let px = (cycle % 40) + 1;
        if reg == px || reg + 1 == px || reg + 2 == px {
            print!("#");
        } else {
            print!(".");
        }

        if (cycle + 1) % 40 == 0 {
            println!();
        }

        if cycle == ops[op_idx].end_cycle {
            reg += ops[op_idx].add;
            op_idx += 1;
        }
    }
    println!();
}

fn main() {
    let input = include_str!("../../assets/day10.txt");
    let ops = load_ops(input);

    sum_cycles_of_interest(&ops);

    render_screen(&ops);
}
