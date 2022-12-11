fn main() {
    let input = include_str!("../../assets/day10.txt");

    let mut cycle = 0;
    let mut reg = 1;
    let mut sum = 0;

    let mut interest = vec![20, 60, 100, 140, 180, 220];
    interest.reverse();

    for line in input.lines() {
        let next = *interest.last().unwrap_or(&i32::MAX);
        if line.starts_with("noop") {
            if cycle + 1 >= next {
                sum += next * reg;
                interest.pop();
            }

            cycle += 1;
        } else {
            let val = line.split(' ').last().unwrap().parse::<i32>().unwrap();

            if cycle + 2 >= next {
                sum += next * reg;
                interest.pop();
            }

            cycle += 2;
            reg += val;
        }
    }

    println!("At end cycle {}, reg {}, sum {}", cycle, reg, sum);
}
