use std::collections::HashSet;
use std::collections::HashMap;

fn estimation_func(val: char) -> u32 {
    let ordinal = val as u32 - 'a' as u32;
    if ordinal > 25 {
        println!("problem value = {}", val);
    }
    (70 - ordinal) * 20
}

fn get_best_option(options: &HashSet<(usize, usize)>, costs: &HashMap<(usize, usize), u32>) -> (usize, usize) {
    let mut best_option = (0, 0);
    let mut best_option_cost = u32::MAX;

    for option in options {
        let current_cost = costs.get(&option).cloned().unwrap_or(u32::MAX - 1);
        if current_cost < best_option_cost {
            best_option_cost = current_cost;
            best_option = option.to_owned();
        }
    }

    best_option
}

fn get_val(node: (usize, usize), map: &Vec<Vec<char>>) -> char {
    map[node.1][node.0]
}

fn get_neighbors(node: (usize, usize), map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut ret: Vec<(usize, usize)> = Vec::new();
    let current_val = get_val(node, map);

    if node.0 > 0 {
        let new_val = get_val((node.0 - 1, node.1), map);
        if new_val as i32 - current_val as i32 <= 1 {
            ret.push((node.0 - 1, node.1));
        }
    }
    if node.0 + 1 < map[0].len() {
        let new_val = get_val((node.0 + 1, node.1), map);
        if new_val as i32 - current_val as i32 <= 1 {
            ret.push((node.0 + 1, node.1));
        }
    }
    if node.1 > 0 {
        let new_val = get_val((node.0, node.1 - 1), map);
        if new_val as i32 - current_val as i32 <= 1 {
            ret.push((node.0, node.1 - 1));
        }
    }
    if node.1 + 1 < map.len() {
        let new_val = get_val((node.0, node.1 + 1), map);
        if new_val as i32 - current_val as i32 <= 1 {
            ret.push((node.0, node.1 + 1));
        }
    }

    ret
}

fn search(start: (usize, usize), end: (usize, usize), map: &Vec<Vec<char>>) -> u32 {
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
    println!("{}", input.len());
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let row: Vec<char> = line.chars().collect();
        map.push(row);
    }
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (i, entry) in map.iter().enumerate() {
        let location = entry.iter().position(|item| { item.clone() == 'S' });
        if let Some(location) = location {
            start = (location, i);
        }

        let location = entry.iter().position(|item| { item.clone() == 'E' });
        if let Some(location) = location {
            end = (location, i);
        }
    }

    map[start.1][start.0] = 'a';
    map[end.1][end.0] = 'z';

    println!("{}, {} => {}, {}", start.0, start.1, end.0, end.1);

    Some(search(start, end, &map))
}

pub fn part_two(input: &str) -> Option<u32> {
    println!("{}", input.len());
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let row: Vec<char> = line.chars().collect();
        map.push(row);
    }
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (i, entry) in map.iter().enumerate() {
        let location = entry.iter().position(|item| { item.clone() == 'S' });
        if let Some(location) = location {
            start = (location, i);
        }

        let location = entry.iter().position(|item| { item.clone() == 'E' });
        if let Some(location) = location {
            end = (location, i);
        }
    }

    map[start.1][start.0] = 'a';
    map[end.1][end.0] = 'z';

    println!("{}, {} => {}, {}", start.0, start.1, end.0, end.1);

    let mut costs: Vec<u32> = Vec::new();
    for i in 0..map.len() {
        let start = (0, i);
        costs.push(search(start, end, &map));
    }

    costs.sort();

    Some(costs[0])
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), None);
    }
}
