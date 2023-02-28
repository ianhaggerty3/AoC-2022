use sscanf::sscanf;
use std::collections::HashSet;
use std::collections::HashMap;
use std::hash::Hash;

fn parse_line(line: &str, options_map: &mut HashMap<String, Vec<String>>, flow_map: &mut HashMap<String, u32>) {
    let parsed = sscanf::sscanf!(line, "Valve {str} has flow rate={u32}; tunnels lead to valves {str}").unwrap();
    let nexts: Vec<&str> = parsed.2.split(", ").collect();
    options_map.insert(parsed.0.clone().to_owned(), nexts.iter().map(|item| item.clone().to_owned() ).collect());
    flow_map.insert(parsed.0.clone().to_owned(), parsed.1);
}

fn get_neighbors(node: &(String, u32), map: &HashMap<String, Vec<(String, u32)>>) -> Vec<(String, u32)> {
    let mut ret: Vec<(String, u32)> = Vec::new();
    let node_key = node.0[node.0.len() - 2..].to_owned();

    let neighbors = map[&node_key].iter().filter(|neighbor| node.0.find(&neighbor.0).is_none());
//    for connected in &map[&node_key] {
    for connected in neighbors {
        let mut new_key = node.0.clone();
        new_key.push_str("|");
        new_key.push_str(&connected.0);
        if node.1 + connected.1 < 30 {
            ret.push((new_key, node.1 + connected.1));
        }
    }

    ret
}

fn get_cost(node: &(String, u32), flow_map: &HashMap<String, u32>) -> i32 {
    let node_key = node.0[node.0.len() - 2..].to_owned();
    let flow_rate = flow_map[&node_key];

    -(flow_rate as i32 * (30 - node.1) as i32)
}

fn search(start: String, map: &HashMap<String, Vec<(String, u32)>>, flow_map: &HashMap<String, u32>) -> i32 {
    let mut open_set: HashSet<(String, u32)> = HashSet::new();
    let mut actual_cost: HashMap<(String, u32), i32> = HashMap::new();
    let mut best_path_flow = 0;
    let mut best_path_node = ("".to_owned(), 0);
    open_set.insert((start.clone(), 0));
    actual_cost.insert((start.clone(), 0), 0);

    while !open_set.is_empty() {
        let current = open_set.iter().next().unwrap().clone();
        open_set.remove(&current);

        for neighbor in get_neighbors(&current, map) {
            // println!("got neighbor {:?} for current {:?}", neighbor, current);
            let new_actual_cost = actual_cost.get(&current).unwrap() + get_cost(&neighbor, flow_map);
            if new_actual_cost < actual_cost.get(&neighbor).cloned().unwrap_or(i32::MAX) {
                actual_cost.insert(neighbor.clone(), new_actual_cost);
                open_set.insert(neighbor.clone());
                if new_actual_cost < best_path_flow {
                    best_path_flow = new_actual_cost;
                    best_path_node = neighbor.clone();
                }
            }
        }
    }

    println!("{:?}", best_path_node);

    -best_path_flow
}

fn get_best_option(open_set: &HashSet<String>, distance: &HashMap<String, u32>) -> String {
    open_set.iter().min_by(|a, b| distance[a.clone()].clone().cmp(&distance[b.clone()].clone())).unwrap().clone()
}

fn search_dist(start: &String, end: &String, map: &HashMap<String, Vec<String>>) -> u32 {
    let mut distance: HashMap<String, u32> = HashMap::new();
    let mut open_set: HashSet<String> = HashSet::new();
    open_set.insert(start.clone());
    distance.insert(start.clone(), 0);

    while !open_set.is_empty() {
        let current = get_best_option(&open_set, &distance);
        open_set.remove(&current);
        if current.clone() == end.clone() {
            return distance[&current] + 1;
        }

        let neighbors = &map[&current];
        for neighbor in neighbors {
            let mut new_cost = distance[&current] + 1;
            if new_cost < distance.get(neighbor).cloned().unwrap_or(u32::MAX) {
                distance.insert(neighbor.clone(), new_cost);
                open_set.insert(neighbor.clone());
            }
        }
    }
    panic!("no path found");
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut options_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut flow_map: HashMap<String, u32> = HashMap::new();
    let start = "AA";


    for (i, line) in input.lines().enumerate() {
        parse_line(line, &mut options_map, &mut flow_map);
    }

    let viable_options: HashSet<String> = options_map.iter()
        .filter(|option| flow_map[option.0] != 0)
        .map(|option| option.0.clone()).collect();

    let mut viable_paths: HashMap<String, Vec<(String, u32)>> = HashMap::new();

    viable_paths.insert(start.to_owned(), Vec::new());
    for option in &viable_options {
        viable_paths.insert(option.clone(), Vec::new());
        viable_paths.get_mut(start).unwrap().push((option.clone(), search_dist(&start.to_owned(), option, &options_map)));
        for other in &viable_options {
            if option == other {
                continue;
            }
            viable_paths.get_mut(option).unwrap().push((other.clone(), search_dist(option, other, &options_map)));
        }
    }

    Some(search(start.to_owned(), &viable_paths, &flow_map))
}

fn get_dual_cost(current: &(String, u32, String, u32), neighbor: &(String, u32, String, u32), flow_map: &HashMap<String, u32>) -> i32 {

    let a_key = neighbor.0[neighbor.0.len() - 2..].to_owned();
    let b_key = neighbor.2[neighbor.2.len() - 2..].to_owned();

    let mut a_rate = flow_map[&a_key];
    let mut b_rate = flow_map[&b_key];

    let a_same = current.0 == neighbor.0;
    let b_same = current.2 == neighbor.2;

    if a_same {
        a_rate = 0;
    }

    if b_same {
        b_rate = 0;
    }

    -(a_rate as i32 * (26 - neighbor.1) as i32) + -(b_rate as i32 * (26 - neighbor.3) as i32)
}

fn get_dual_neighbors(node: &(String, u32, String, u32), map: &HashMap<String, Vec<(String, u32)>>) -> Vec<(String, u32, String, u32)> {
    let mut ret: Vec<(String, u32, String, u32)> = Vec::new();
    let a_key = node.0[node.0.len() - 2..].to_owned();
    let b_key = node.2[node.2.len() - 2..].to_owned();

    let a_neighbors = map[&a_key].iter().filter(|neighbor| node.0.find(&neighbor.0).is_none() && node.2.find(&neighbor.0).is_none());
    for a_neighbor in a_neighbors {
        let mut new_a_path = node.0.clone();
        new_a_path.push_str("|");
        new_a_path.push_str(&a_neighbor.0);
        
        // neighbors where both move
        let b_neighbors = map[&b_key].iter().filter(|neighbor| new_a_path.find(&neighbor.0).is_none() && node.2.find(&neighbor.0).is_none());
        for b_neighbor in b_neighbors {
            let mut new_b_path = node.2.clone();
            new_b_path.push_str("|");
            new_b_path.push_str(&b_neighbor.0);
            if node.1 + a_neighbor.1 < 26 && node.3 + b_neighbor.1 < 26 {
                ret.push((new_a_path.clone(), node.1 + a_neighbor.1, new_b_path.clone(), node.3 + b_neighbor.1));
            }
        }

        // neighbors where one moves
        let b_neighbors = map[&b_key].iter().filter(|neighbor| node.0.find(&neighbor.0).is_none() && node.2.find(&neighbor.0).is_none());
        for b_neighbor in b_neighbors {
            let mut new_b_path = node.2.clone();
            new_b_path.push_str("|");
            new_b_path.push_str(&b_neighbor.0);

            if node.3 + b_neighbor.1 < 26 {
                ret.push((node.0.clone(), node.1, new_b_path, node.3 + b_neighbor.1))
            }
        }

        if node.1 + a_neighbor.1 < 26 {
            ret.push((new_a_path, node.1 + a_neighbor.1, node.2.clone(), node.3));
        }
    }

    ret
}

fn get_best_dual_option(options: &HashSet<(String, u32, String, u32)>, costs: &HashMap<(String, u32, String, u32), i32>) -> (String, u32, String, u32) {
    let mut best_option = ("".to_owned(), 0, "".to_owned(), 0);
    let mut best_option_cost = i32::MAX;

    for option in options {
        let current_cost = costs.get(option).cloned().unwrap();
        if current_cost < best_option_cost {
            best_option_cost = current_cost;
            best_option = option.clone().to_owned();
        }
    }

    best_option
}

fn dual_search(start: String, map: &HashMap<String, Vec<(String, u32)>>, flow_map: &HashMap<String, u32>) -> i32 {
    let mut open_set: HashSet<(String, u32, String, u32)> = HashSet::new();
    let mut actual_cost: HashMap<(String, u32, String, u32), i32> = HashMap::new();
    let mut best_path_flow = 0;
    let mut best_path_node = ("".to_owned(), 0, "".to_owned(), 0);
    open_set.insert((start.clone(), 0, start.clone(), 0));
    actual_cost.insert((start.clone(), 0, start.clone(), 0), 0);

    while !open_set.is_empty() {
        let current = get_best_dual_option(&open_set, &actual_cost);
        open_set.remove(&current);

        for neighbor in get_dual_neighbors(&current, map) {
            // println!("got neighbor {:?} for current {:?}", neighbor, current);
            let new_actual_cost = actual_cost.get(&current).unwrap() + get_dual_cost(&current, &neighbor, flow_map);
            if new_actual_cost < actual_cost.get(&neighbor).cloned().unwrap_or(i32::MAX) {
                actual_cost.insert(neighbor.clone(), new_actual_cost);
                open_set.insert(neighbor.clone());
                if new_actual_cost < best_path_flow {
                    best_path_flow = new_actual_cost;
                    best_path_node = neighbor.clone();
                }
            }
        }
    }

    println!("{:?}", best_path_node);

    -best_path_flow
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut options_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut flow_map: HashMap<String, u32> = HashMap::new();
    let start = "AA";


    for line in input.lines() {
        parse_line(line, &mut options_map, &mut flow_map);
    }

    let viable_options: HashSet<String> = options_map.iter()
        .filter(|option| flow_map[option.0] != 0)
        .map(|option| option.0.clone()).collect();

    let mut viable_paths: HashMap<String, Vec<(String, u32)>> = HashMap::new();

    viable_paths.insert(start.to_owned(), Vec::new());

    for option in &viable_options {
        viable_paths.insert(option.clone(), Vec::new());
        viable_paths.get_mut(start).unwrap().push((option.clone(), search_dist(&start.to_owned(), option, &options_map)));
        for other in &viable_options {
            if option == other {
                continue;
            }
            viable_paths.get_mut(option).unwrap().push((other.clone(), search_dist(option, other, &options_map)));
        }
    }

    println!("got to dual_search");

    Some(dual_search(start.to_owned(), &viable_paths, &flow_map))
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
