use std::cmp::Ordering;
use std::collections::HashSet;
use std::collections::HashMap;
use itertools::Itertools;

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

fn search_dist(start: &String, end: &String, map: &HashMap<String, Vec<String>>, flow_map: &HashMap<String, u32>) -> Option<u32> {
    let mut distance: HashMap<String, u32> = HashMap::new();
    let mut open_set: HashSet<String> = HashSet::new();
    open_set.insert(start.clone());
    distance.insert(start.clone(), 0);

    while !open_set.is_empty() {
        let current = get_best_option(&open_set, &distance);
        open_set.remove(&current);
        if current.clone() == end.clone() {
            return Some(distance[&current] + 1);
        }

        // this optimization doesn't work in this form. would need to do it during traversal
        //if flow_map[&current] > 3 {
        //    continue;
        //}

        let neighbors = &map[&current];
        for neighbor in neighbors {
            let new_cost = distance[&current] + 1;
            if new_cost < distance.get(neighbor).cloned().unwrap_or(u32::MAX) {
                distance.insert(neighbor.clone(), new_cost);
                open_set.insert(neighbor.clone());
            }
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<i32> {
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
        let start_dist = search_dist(&start.to_owned(), option, &options_map, &flow_map);
        viable_paths.insert(option.clone(), Vec::new());
        if let Some(start_dist) = start_dist {
            viable_paths.get_mut(start).unwrap().push((option.clone(), start_dist));
        }

        for other in &viable_options {
            if option == other {
                continue;
            }
            let option_dist = search_dist(option, other, &options_map, &flow_map);
            if let Some(option_dist) = option_dist {
                viable_paths.get_mut(option).unwrap().push((other.clone(), option_dist));
            }
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

    // neighbors where both move
    // let a_neighbors = map[&a_key].iter()
    //     .filter(|neighbor| node.0.find(&neighbor.0).is_none() && node.2.find(&neighbor.0).is_none())
    //     .filter(|neighbor| node.1 + neighbor.1 < 26); 
    // for a_neighbor in a_neighbors {
    //     let mut new_a_path = node.0.clone();
    //     new_a_path.push_str("|");
    //     new_a_path.push_str(&a_neighbor.0);
    //     
    //     let b_neighbors = map[&b_key].iter()
    //         .filter(|neighbor| new_a_path.find(&neighbor.0).is_none() && node.2.find(&neighbor.0).is_none())
    //         .filter(|neighbor| node.3 + neighbor.1 < 26);
    //     for b_neighbor in b_neighbors {
    //         let mut new_b_path = node.2.clone();
    //         new_b_path.push_str("|");
    //         new_b_path.push_str(&b_neighbor.0);
    //         ret.push((new_a_path.clone(), node.1 + a_neighbor.1, new_b_path.clone(), node.3 + b_neighbor.1));
    //     }
    // }

    // proposal: if any "double" move exists, it is never beneifical to only make one move
    // likely not true in the general case, would need more checks
    // this proposal seems false
    // if doubles_ever {
    //     return ret;
    // }

    // neighbors where one moves
    let a_neighbors = map[&a_key].iter().filter(|neighbor| node.0.find(&neighbor.0).is_none() && node.2.find(&neighbor.0).is_none());
    for a_neighbor in a_neighbors {
        let mut new_a_path = node.0.clone();
        new_a_path.push_str("|");
        new_a_path.push_str(&a_neighbor.0);

        if node.1 + a_neighbor.1 < 26 {
            ret.push((new_a_path, node.1 + a_neighbor.1, node.2.clone(), node.3));
        }
    }

    let b_neighbors = map[&b_key].iter().filter(|neighbor| node.0.find(&neighbor.0).is_none() && node.2.find(&neighbor.0).is_none());
    for b_neighbor in b_neighbors {
        let mut new_b_path = node.2.clone();
        new_b_path.push_str("|");
        new_b_path.push_str(&b_neighbor.0);

        if node.3 + b_neighbor.1 < 26 {
            ret.push((node.0.clone(), node.1, new_b_path, node.3 + b_neighbor.1))
        }
    }

    ret
}

fn reorganize_path(path: String) -> String {
    let mut parts: Vec<_> = path.split('|').collect();
    parts.sort();
    let ret: String = parts.iter().cloned().intersperse("|").collect();
    ret
}

fn reorganize_node(node: (String, u32, String, u32)) -> (String, u32, String, u32) {
    let a_path = reorganize_path(node.0.clone());
    let b_path = reorganize_path(node.2.clone());
    if a_path.cmp(&b_path) == Ordering::Less {
        (a_path, node.1, b_path, node.3)
    } else {
        (b_path, node.3, a_path, node.1)
    }
}

fn dual_search(start: String, map: &HashMap<String, Vec<(String, u32)>>, flow_map: &HashMap<String, u32>) -> i32 {
    let mut open_set: HashSet<(String, u32, String, u32)> = HashSet::new();
    let mut actual_cost: HashMap<(String, u32, String, u32), i32> = HashMap::new();
    let mut best_path_flow = 0;
    let mut best_path_node = ("".to_owned(), 0, "".to_owned(), 0);
    open_set.insert((start.clone(), 0, start.clone(), 0));
    actual_cost.insert((start.clone(), 0, start.clone(), 0), 0);

    while !open_set.is_empty() {
        let current = open_set.iter().next().unwrap().clone();
        open_set.remove(&current);
        let reorganized_current = reorganize_node(current.clone());

        for neighbor in get_dual_neighbors(&current, map) {
            let reorganized_neighbor = reorganize_node(neighbor.clone());
            let dual_cost = get_dual_cost(&current, &neighbor, flow_map);
            let new_actual_cost = actual_cost.get(&reorganized_current).unwrap() + dual_cost;

            if new_actual_cost < actual_cost.get(&reorganized_neighbor).cloned().unwrap_or(i32::MAX) {
                actual_cost.insert(reorganized_neighbor.clone(), new_actual_cost);
                open_set.insert(neighbor.clone());
                if new_actual_cost < best_path_flow {
                    best_path_flow = new_actual_cost;
                    best_path_node = neighbor.clone();
                    println!("new best_path_flow = {}", best_path_flow);
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
        let start_dist = search_dist(&start.to_owned(), option, &options_map, &flow_map);
        viable_paths.insert(option.clone(), Vec::new());
        if let Some(start_dist) = start_dist {
            viable_paths.get_mut(start).unwrap().push((option.clone(), start_dist));
        }
        for other in &viable_options {
            if option == other {
                continue;
            }
            let option_dist = search_dist(option, other, &options_map, &flow_map);
            if let Some(option_dist) = option_dist {
                viable_paths.get_mut(option).unwrap().push((other.clone(), option_dist));
            }
        }
    }

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
