use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone, Debug)]
enum Monkey {
    Operation(String, String, Op),
    Number(i64),
}

fn parse_operand(operand: char) -> Op {
    match operand {
        '+' => Op::Add,
        '-' => Op::Sub,
        '*' => Op::Mul,
        '/' => Op::Div,
        _ => panic!("Unrecognized operand"),
    }
}

fn parse_line(line: &str) -> (String, Monkey) {
    let total_parts: Vec<&str> = line.split(":").collect();
    let monkey_parts: Vec<&str> = total_parts[1].trim().split(" ").collect();
    if monkey_parts.len() == 3 {
        (total_parts[0].clone().to_string(), Monkey::Operation(monkey_parts[0].to_string(), monkey_parts[2].to_string(), parse_operand(monkey_parts[1].chars().nth(0).unwrap()))) 
    } else {
        (total_parts[0].clone().to_string(), Monkey::Number(monkey_parts[0].parse::<i64>().unwrap()))
    }
}

fn do_operation(a: i64, b: i64, op: &Op) -> i64 {
    match op {
        Op::Add => a + b,
        Op::Mul => a * b,
        Op::Sub => a - b,
        Op::Div => a / b,
    }
}

fn get_val(id: &String, monkey_lookup: &HashMap<String, Monkey>) -> i64 {
    let monkey = &monkey_lookup[id];
    match monkey {
        Monkey::Operation(a, b, op) => do_operation(get_val(&a, monkey_lookup), get_val(&b, monkey_lookup), op),
        Monkey::Number(num) => *num, 
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut monkey_lookup = HashMap::new();

    for raw_monkey in input.lines() {
        let (id, monkey) = parse_line(raw_monkey);
        println!("id = {} monkey = {:?}", id, monkey);
        monkey_lookup.insert(id, monkey);
    }
    Some(get_val(&"root".to_string(), &monkey_lookup))
}

pub fn part_two(input: &str) -> Option<i64> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), None);
    }
}
