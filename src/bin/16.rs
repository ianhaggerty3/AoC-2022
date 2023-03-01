use std::collections::HashSet;
use std::collections::HashMap;

fn get_neighbors(node: &(u64, u64, u64), map: &HashMap<u64, Vec<(u64, u64)>>) -> Vec<(u64, u64, u64)> {
    let mut ret: Vec<(u64, u64, u64)> = Vec::new();

    let neighbors = map[&node.2].iter().filter(|neighbor| (node.0 & neighbor.0 == 0));
    for neighbor in neighbors {
        let new_path = node.0 | neighbor.0;
        if node.1 + neighbor.1 < 30 {
            ret.push((new_path, node.1 + neighbor.1, neighbor.0));
        }
    }

    ret
}

fn get_cost(node: &(u64, u64, u64), flow_map: &HashMap<u64, u64>) -> i32 {
    let flow_rate = flow_map[&node.2];

    -(flow_rate as i32 * (30 - node.1) as i32)
}

fn search(start: u64, map: &HashMap<u64, Vec<(u64, u64)>>, flow_map: &HashMap<u64, u64>) -> i32 {
    let mut open_set: HashSet<(u64, u64, u64)> = HashSet::new();
    let mut actual_cost: HashMap<(u64, u64, u64), i32> = HashMap::new();
    let mut best_path_flow = 0;
    open_set.insert((start, 0, start));
    actual_cost.insert((start, 0, start), 0);

    while !open_set.is_empty() {
        let current = open_set.iter().next().unwrap().clone();
        open_set.remove(&current);

        for neighbor in get_neighbors(&current, map) {
            let new_actual_cost = actual_cost.get(&current).unwrap() + get_cost(&neighbor, flow_map);
            if new_actual_cost < actual_cost.get(&neighbor).cloned().unwrap_or(i32::MAX) {
                actual_cost.insert(neighbor.clone(), new_actual_cost);
                open_set.insert(neighbor.clone());
                if new_actual_cost < best_path_flow {
                    best_path_flow = new_actual_cost;
                }
            }
        }
    }

    -best_path_flow
}

fn get_best_option(open_set: &HashSet<u64>, distance: &HashMap<u64, u64>) -> u64 {
    open_set.iter().min_by(|a, b| distance[a].clone().cmp(&distance[b].clone())).unwrap().clone()
}

fn search_dist(start: u64, end: u64, map: &HashMap<u64, Vec<u64>>, flow_map: &HashMap<u64, u64>) -> Option<u64> {
    let mut distance: HashMap<u64, u64> = HashMap::new();
    let mut open_set: HashSet<u64> = HashSet::new();
    open_set.insert(start);
    distance.insert(start, 0);

    while !open_set.is_empty() {
        let current = get_best_option(&open_set, &distance);
        open_set.remove(&current);
        if current == end {
            return Some(distance[&current] + 1);
        }

        let neighbors = &map[&current];
        for neighbor in neighbors {
            let new_cost = distance[&current] + 1;
            if new_cost < distance.get(neighbor).cloned().unwrap_or(u64::MAX) {
                distance.insert(neighbor.clone(), new_cost);
                open_set.insert(neighbor.clone());
            }
        }
    }

    None
}

fn parse_input(input: &str, options_map: &mut HashMap<u64, Vec<u64>>, flow_map: &mut HashMap<u64, u64>) -> u64 {
    let mut id_to_int: HashMap<&str, u64> = HashMap::new();
    let mut ret = 0;

    for (i, line) in input.lines().enumerate() {
        let parsed = sscanf::sscanf!(line, "Valve {str} has flow rate={u64}; tunnels lead to valves {str}").unwrap();
        id_to_int.insert(parsed.0, 1 << i);
        if parsed.0.eq("AA") {
            ret = (1 << i) as u64;
        }
    }

    for line in input.lines() {
        let parsed = sscanf::sscanf!(line, "Valve {str} has flow rate={u64}; tunnels lead to valves {str}").unwrap();
        let nexts: Vec<&str> = parsed.2.split(", ").collect();
        options_map.insert(id_to_int[&parsed.0], nexts.iter().map(|item| id_to_int[item]).collect());
        flow_map.insert(id_to_int[&parsed.0], parsed.1);
    }

    ret
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut options_map: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut flow_map: HashMap<u64, u64> = HashMap::new();
    let start = parse_input(input, &mut options_map, &mut flow_map);

    let viable_options: HashSet<u64> = options_map.iter()
        .filter(|option| flow_map[option.0] != 0)
        .map(|option| option.0.clone()).collect();

    let mut viable_paths: HashMap<u64, Vec<(u64, u64)>> = HashMap::new();

    viable_paths.insert(start, Vec::new());
    for option in &viable_options {
        let start_dist = search_dist(start, *option, &options_map, &flow_map);
        viable_paths.insert(option.clone(), Vec::new());
        if let Some(start_dist) = start_dist {
            viable_paths.get_mut(&start).unwrap().push((option.clone(), start_dist));
        }

        for other in &viable_options {
            if option == other {
                continue;
            }
            let option_dist = search_dist(*option, *other, &options_map, &flow_map);
            if let Some(option_dist) = option_dist {
                viable_paths.get_mut(option).unwrap().push((other.clone(), option_dist));
            }
        }
    }

    Some(search(start, &viable_paths, &flow_map))
}

fn get_dual_cost(current: &(u64, u64, u64, u64, u64, u64), neighbor: &(u64, u64, u64, u64, u64, u64), flow_map: &HashMap<u64, u64>) -> i32 {
    let mut a_rate = flow_map[&current.2];
    let mut b_rate = flow_map[&current.5];

    let a_same = current.2 == neighbor.2;
    let b_same = current.5 == neighbor.5;

    if a_same {
        a_rate = 0;
    }

    if b_same {
        b_rate = 0;
    }

    -(a_rate as i32 * (26 - neighbor.1) as i32) + -(b_rate as i32 * (26 - neighbor.4) as i32)
}

fn get_dual_neighbors(node: &(u64, u64, u64, u64, u64, u64), map: &HashMap<u64, Vec<(u64, u64)>>) -> Vec<(u64, u64, u64, u64, u64, u64)> {
    let mut ret: Vec<(u64, u64, u64, u64, u64, u64)> = Vec::new();

    // neighbors where one moves
    let neighbors = map[&node.2].iter().filter(|neighbor| (node.0 & neighbor.0 == 0) && (node.3 & neighbor.0 == 0));
    for neighbor in neighbors {
        let new_path = node.0 | neighbor.0;

        if node.1 + neighbor.1 < 26 {
            ret.push((new_path, node.1 + neighbor.1, neighbor.0, node.3, node.4, node.5));
        }
    }

    let neighbors = map[&node.5].iter().filter(|neighbor| (node.0 & neighbor.0 == 0) && (node.3 & neighbor.0 == 0));
    for neighbor in neighbors {
        let new_path = node.3 | neighbor.0;

        if node.4 + neighbor.1 < 26 {
            ret.push((node.0, node.1, node.2, new_path, node.4 + neighbor.1, neighbor.0))
        }
    }

    ret
}

fn dual_search(start: u64, map: &HashMap<u64, Vec<(u64, u64)>>, flow_map: &HashMap<u64, u64>) -> i32 {
    let mut open_set: HashSet<(u64, u64, u64, u64, u64, u64)> = HashSet::new();
    let mut actual_cost: HashMap<(u64, u64, u64, u64, u64, u64), i32> = HashMap::new();
    let mut best_path_flow = 0;
    open_set.insert((start, 0, start, start, 0, start));
    actual_cost.insert((start, 0, start, start, 0, start), 0);

    while !open_set.is_empty() {
        let current = open_set.iter().next().unwrap().clone();
        open_set.remove(&current);

        for neighbor in get_dual_neighbors(&current, map) {
            let dual_cost = get_dual_cost(&current, &neighbor, flow_map);
            let new_actual_cost = actual_cost.get(&current).unwrap() + dual_cost;

            if new_actual_cost < actual_cost.get(&neighbor).cloned().unwrap_or(i32::MAX) {
                actual_cost.insert(neighbor.clone(), new_actual_cost);
                open_set.insert(neighbor.clone());
                if new_actual_cost < best_path_flow {
                    best_path_flow = new_actual_cost;
                    println!("new best_path_flow = {}", best_path_flow);
                }
            }
        }
    }

    -best_path_flow
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut options_map: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut flow_map: HashMap<u64, u64> = HashMap::new();
    let start = parse_input(input, &mut options_map, &mut flow_map);

    let viable_options: HashSet<u64> = options_map.iter()
        .filter(|option| flow_map[option.0] != 0)
        .map(|option| option.0.clone()).collect();

    let mut viable_paths: HashMap<u64, Vec<(u64, u64)>> = HashMap::new();

    viable_paths.insert(start, Vec::new());

    for option in &viable_options {
        let start_dist = search_dist(start, *option, &options_map, &flow_map);
        viable_paths.insert(option.clone(), Vec::new());

        if let Some(start_dist) = start_dist {
            viable_paths.get_mut(&start).unwrap().push((option.clone(), start_dist));
        }

        for other in &viable_options {
            if option == other {
                continue;
            }
            let option_dist = search_dist(*option, *other, &options_map, &flow_map);
            if let Some(option_dist) = option_dist {
                viable_paths.get_mut(option).unwrap().push((other.clone(), option_dist));
            }
        }
    }

    Some(dual_search(start, &viable_paths, &flow_map))
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
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), Some(1707));
    }
}
