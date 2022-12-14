#[derive(Debug)]
#[derive(Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: char,
    operand: Option<u64>,
    divisor: u64,
    true_index: usize,
    false_index: usize,
    times_inspected: u64,
}

fn parse_monkey(paragraph: Vec<& str>) -> Monkey {
    let parts: Vec<_> = paragraph[0].split(":").collect();
    let items: Vec<_> = parts[1].split(",").collect();
    let items: Vec<_> = items.iter().map(|item| item.to_owned().trim().parse::<u64>().unwrap()).collect();
    let mut operation = '*';
    if paragraph[1].contains("+") {
        operation = '+';
    }
    let mut parts: Vec<_> = paragraph[1].split(" ").collect();
    parts.reverse();
    let potential_operand = parts[0].parse::<u64>();
    let mut operand: Option<u64> = None;
    if potential_operand.is_ok() {
        operand = Some(potential_operand.unwrap());
    }
    let mut parts: Vec<_> = paragraph[2].split(" ").collect();
    parts.reverse();
    let divisor = parts[0].parse::<u64>().unwrap();
    let mut parts: Vec<_> = paragraph[3].split(" ").collect();
    parts.reverse();
    let true_index = parts[0].parse::<usize>().unwrap();
    let mut parts: Vec<_> = paragraph[4].split(" ").collect();
    parts.reverse();
    let false_index = parts[0].parse::<usize>().unwrap();

    Monkey {
        items: items.clone(),
        operation,
        operand,
        divisor,
        true_index,
        false_index,
        times_inspected: 0,
    }
}

fn perform_operation(item: u64, operation: char, operand: Option<u64>) -> u64 {
    let operand = if let Some(val) = operand {
        val
    } else {
        item
    };

    if operation == '+' {
        item + operand
    } else {
        item * operand
    }
}

fn sim_round(monkeys: &mut Vec<Monkey>) {
    for i in 0..monkeys.len() {
        let monkey = &mut monkeys[i];
        monkey.times_inspected += monkey.items.len() as u64;
        let items = monkey.items.clone();
        monkey.items.clear();
        let monkey = monkeys[i].clone();
        for item in items {
            let current_item = perform_operation(item, monkey.operation, monkey.operand);
            let current_item = current_item / 3;
            if current_item % monkey.divisor == 0 {
                monkeys[monkey.true_index].items.push(current_item);
            } else {
                monkeys[monkey.false_index].items.push(current_item);
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut monkey_lines: Vec<usize> = Vec::new();
    for (i, line) in input.lines().enumerate() {
        if line.starts_with("Monkey") {
            monkey_lines.push(i);
        }
    }

    let lines: Vec<_> = input.lines().collect();
    let mut monkeys: Vec<Monkey> = Vec::new();
    for monkey_index in monkey_lines {
        let monkey_text = lines[(monkey_index + 1)..(monkey_index + 6)].to_vec();
        monkeys.push(parse_monkey(monkey_text));
    }

    println!("{:?}", monkeys);

    for i in 0..20 {
        sim_round(&mut monkeys);
    }

    monkeys.sort_by(|a, b| a.times_inspected.partial_cmp(&b.times_inspected).unwrap());
    monkeys.reverse();

    Some(monkeys[0].times_inspected * monkeys[1].times_inspected)
}

fn sim_round_worried(monkeys: &mut Vec<Monkey>, common_denominator: u64) {
    for i in 0..monkeys.len() {
        let monkey = &mut monkeys[i];
        monkey.times_inspected += monkey.items.len() as u64;
        let items = monkey.items.clone();
        monkey.items.clear();
        let monkey = monkeys[i].clone();
        for item in items {
            let current_item = perform_operation(item, monkey.operation, monkey.operand);
            let current_item = current_item % common_denominator;
            if current_item % monkey.divisor == 0 {
                monkeys[monkey.true_index].items.push(current_item);
            } else {
                monkeys[monkey.false_index].items.push(current_item);
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {

    let mut monkey_lines: Vec<usize> = Vec::new();
    for (i, line) in input.lines().enumerate() {
        if line.starts_with("Monkey") {
            monkey_lines.push(i);
        }
    }

    let lines: Vec<_> = input.lines().collect();
    let mut monkeys: Vec<Monkey> = Vec::new();
    for monkey_index in monkey_lines {
        let monkey_text = lines[(monkey_index + 1)..(monkey_index + 6)].to_vec();
        monkeys.push(parse_monkey(monkey_text));
    }

    let mut common_denominator = 1;
    for monkey in &monkeys {
        common_denominator *= monkey.divisor;
    }

    println!("{}", common_denominator);

    println!("{:?}", monkeys);

    for i in 0..10_000 {
        sim_round_worried(&mut monkeys, common_denominator);
    }

    monkeys.sort_by(|a, b| a.times_inspected.partial_cmp(&b.times_inspected).unwrap());
    monkeys.reverse();

    Some(monkeys[0].times_inspected * monkeys[1].times_inspected)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), None);
    }
}
