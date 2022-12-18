use sscanf::sscanf;
use std::collections::HashSet;
use std::collections::HashMap;

fn parse_line(line: &str, options_map: &mut HashMap<String, Vec<String>>, flow_map: &mut HashMap<String, u32>) {
    let parsed = sscanf::sscanf!(line, "Valve {str} has flow rate={u32}; tunnels lead to valves {str}").unwrap();
    let nexts: Vec<&str> = parsed.2.split(", ").collect();
    options_map.insert(parsed.0.clone().to_owned(), nexts.iter().map(|item| item.clone().to_owned() ).collect());
    flow_map.insert(parsed.0.clone().to_owned(), parsed.1);
    // println!("{:?}", nexts);
    // println!("{:?}", options_map);
}

// fn get_best_option(options: &HashSet<(usize, usize)>, costs: &HashMap<(usize, usize), u32>) -> (usize, usize) {
//     let mut best_option = (0, 0);
//     let mut best_option_cost = u32::MAX;
//
//     for option in options {
//         let current_cost = costs.get(&option).cloned().unwrap_or(u32::MAX - 1);
//         if current_cost < best_option_cost {
//             best_option_cost = current_cost;
//             best_option = option.to_owned();
//         }
//     }
//
//     best_option
// }
//
// fn get_val(node: (usize, usize), map: &Vec<Vec<char>>) -> char {
//     map[node.1][node.0]
// }
//
fn search(start: String, map: &Vec<Vec<char>>) -> u32 {
    for i in 0..30 {

    }
    let mut open_set: HashSet<(usize, usize)> = HashSet::new();
    // let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut actual_cost: HashMap<(usize, usize), u32> = HashMap::new();
    let mut estimated_cost: HashMap<(usize, usize), u32> = HashMap::new();
    open_set.insert(start);
    actual_cost.insert(start, 0);
    estimated_cost.insert(start, estimation_func('a'));
    estimated_cost.insert(end, estimation_func('z'));

    while !open_set.is_empty() {
        let current = get_best_option(&open_set, &estimated_cost);
        open_set.remove(&current);
        if current == end {
            return actual_cost.get(&current).unwrap().clone();
        }

        for neighbor in get_neighbors(current, map) {
            let new_actual_cost = actual_cost.get(&current).unwrap() + 1;
            if new_actual_cost < actual_cost.get(&neighbor).cloned().unwrap_or(u32::MAX) {
                actual_cost.insert(neighbor, new_actual_cost);
                estimated_cost.insert(neighbor, new_actual_cost + estimation_func(map[neighbor.1][neighbor.0]));
                open_set.insert(neighbor);
            }
        }


    }

    panic!("end not found");
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut options_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut flow_map: HashMap<String, u32> = HashMap::new();

    for line in input.lines() {
        parse_line(line, &mut options_map, &mut flow_map);
    }

    None
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
