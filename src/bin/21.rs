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
        monkey_lookup.insert(id, monkey);
    }
    Some(get_val(&"root".to_string(), &monkey_lookup))
}

fn get_num_to_equal(id: &String, monkey_lookup: &HashMap<String, Monkey>) -> Option<i64> {
    let monkey = &monkey_lookup[id];
    if id.eq(&"humn".to_string()) {
        return None;
    }

    match monkey {
        Monkey::Operation(left, right, op) => {
           let left_val = get_num_to_equal(left, monkey_lookup);
           let right_val = get_num_to_equal(right, monkey_lookup);
           if left_val.is_none() || right_val.is_none() {
               return None;
           } else {
               return Some(do_operation(left_val.unwrap(), right_val.unwrap(), op));
           }
        },
        Monkey::Number(num) => {
            Some(num.clone())
        },
    }
}

fn find_unknown(known: i64, total: i64, op: &Op, first: bool) -> i64 {
   match op {
       Op::Add => total - known,
       Op::Sub => {
           if first {
               known - total
           } else {
               total + known
           }
       },
       Op::Mul => total / known,
       Op::Div => {
           if first {
               known / total
           } else {
               known * total
           }
       },
   }
}

fn get_missing_num(id: &String, cur_eq: i64, monkey_lookup: &HashMap<String, Monkey>) -> i64 {
    if id.eq(&"humn".to_string()) {
        return cur_eq;
    }

    let monkey = &monkey_lookup[id];
    match monkey {
        Monkey::Operation(left, right, op) => {
            let left_val = get_num_to_equal(left, monkey_lookup);
            let right_val = get_num_to_equal(right, monkey_lookup);
            if left_val.is_none() {
                let num_to_eq = find_unknown(right_val.unwrap(), cur_eq, op, false);
                println!("new num to eq = {}", num_to_eq);
                return get_missing_num(left, num_to_eq, monkey_lookup);
            } else if right_val.is_none() {
                let num_to_eq = find_unknown(left_val.unwrap(), cur_eq, op, true);
                println!("new num to eq = {}", num_to_eq);
                return get_missing_num(right, num_to_eq, monkey_lookup);
            } else {
                panic!("Unexpected tree path");
            }
        },
        _ => todo!(),
    }
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut monkey_lookup = HashMap::new();

    for raw_monkey in input.lines() {
        let (id, monkey) = parse_line(raw_monkey);
        monkey_lookup.insert(id, monkey);
    }

    // Assumes that both sides do not have the human on them, at any point.
    let root_monkey = &monkey_lookup[&"root".to_string()];
    match root_monkey {
        Monkey::Operation(left, right, _op) => {
            if let Some(first_eq) = get_num_to_equal(left, &monkey_lookup) {
                println!("first number to eq 1 = {}", first_eq);
                return Some(get_missing_num(right, first_eq, &monkey_lookup));
            } else if let Some(first_eq) = get_num_to_equal(right, &monkey_lookup) {
                println!("first number to eq 2 = {}", first_eq);
                return Some(get_missing_num(left, first_eq, &monkey_lookup));
            } else {
                panic!("Unknown Information on both sides!");
            }
        },
        _ => todo!(),
    }
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
        assert_eq!(part_two(&input), Some(301));
    }
}
