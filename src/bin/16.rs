use sscanf::sscanf;
use std::collections::HashSet;
use std::collections::HashMap;
use std::hash::Hash;

fn parse_line(line: &str, options_map: &mut HashMap<String, Vec<String>>, flow_map: &mut HashMap<String, u32>) {
    let parsed = sscanf::sscanf!(line, "Valve {str} has flow rate={u32}; tunnels lead to valves {str}").unwrap();
    let nexts: Vec<&str> = parsed.2.split(", ").collect();
    options_map.insert(parsed.0.clone().to_owned(), nexts.iter().map(|item| item.clone().to_owned() ).collect());
    flow_map.insert(parsed.0.clone().to_owned(), parsed.1);
    // println!("{:?}", nexts);
    // println!("{:?}", options_map);
}

fn get_best_option(options: &HashSet<(String, u32)>, costs: &HashMap<(String, u32), i32>) -> (String, u32) {
    let mut best_option = ("".clone().to_owned(), 0);
    let mut best_option_cost = i32::MAX;

    for option in options {
        let current_cost = costs.get(&option).cloned().unwrap();
        if current_cost < best_option_cost {
            best_option_cost = current_cost;
            best_option = option.clone().to_owned();
        }
    }

    best_option
}

fn is_end(node: &(String, u32)) -> bool {
    node.1 == 30
}

fn check_previous(node: &(String, u32), previous: &HashMap<(String, u32), (String, u32)>) -> bool {
    let problem_node = node.0.clone() + "'";

    let mut current = node;
    while previous.contains_key(current) {
        current = &previous[current];
        if current.0 == problem_node {
            println!("already visited {}", node.0);
            return true;
        }
    }

    false
}

fn get_neighbors(node: &(String, u32), map: &HashMap<String, Vec<String>>, previous: &HashMap<(String, u32), (String, u32)>) -> Vec<(String, u32)> {
    let mut ret: Vec<(String, u32)> = Vec::new();

    if !check_previous(node, previous) && !node.0.contains("'") {
        ret.push((node.0.clone() + "'", node.1 + 1));
    }

    let cleaned_node = node.0.clone().replace("'", "");

    for connected in &map[&cleaned_node] {
        ret.push(((connected.clone(), node.1 + 1)))
    }

    ret
}

fn get_cost(node: &(String, u32), flow_map: &HashMap<String, u32>) -> i32 {
    if !node.0.contains("'") {
        return 0;
    }

    let cleaned_node = node.0.clone().replace("'", "");
    let flow_rate = flow_map[&cleaned_node];

    -(flow_rate as i32 * (30 - node.1) as i32)
}

fn search(start: String, map: &HashMap<String, Vec<String>>, flow_map: &HashMap<String, u32>) -> i32 {
    let mut open_set: HashSet<(String, u32)> = HashSet::new();
    let mut previous: HashMap<(String, u32), (String, u32)> = HashMap::new();
    let mut actual_cost: HashMap<(String, u32), i32> = HashMap::new();
    open_set.insert((start.clone(), 1));
    actual_cost.insert((start.clone(), 1), 0);

    while !open_set.is_empty() {
        let current = get_best_option(&open_set, &actual_cost);
        open_set.remove(&current);
        if is_end(&current) {
            println!("found end, {}, {}", current.0, current.1);
            let mut prev = &current;
            while previous.contains_key(prev) {
                prev = &previous[prev];
                println!("before that {}, {}", prev.0, prev.1);
            }

            return actual_cost.get(&current).unwrap().clone();
        }

        for neighbor in get_neighbors(&current, map, &previous) {
            let new_actual_cost = actual_cost.get(&current).unwrap() + get_cost(&neighbor, flow_map);
            if new_actual_cost < actual_cost.get(&neighbor).cloned().unwrap_or(i32::MAX) {
                if neighbor.0.contains("'") {
                    println!("turning on {} on turn {}", neighbor.0, neighbor.1)
                }
                actual_cost.insert(neighbor.clone(), new_actual_cost);
                open_set.insert(neighbor.clone());
                previous.insert(neighbor.clone(), current.clone());
            }
        }
    }

    panic!("end not found");
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut options_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut flow_map: HashMap<String, u32> = HashMap::new();

    for line in input.lines() {
        parse_line(line, &mut options_map, &mut flow_map);
    }

    Some(search("VN".to_owned(), &options_map, &flow_map))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), None);
    }
}
