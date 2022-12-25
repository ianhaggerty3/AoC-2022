use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Blizzard {
    Up(usize, usize),
    Down(usize, usize),
    Left(usize, usize),
    Right(usize, usize),
}

fn get_landscape(blizzards: &Vec<Blizzard>, round: u32, width: usize, height: usize) -> Vec<Blizzard> {
    blizzards.iter()
        .map(|blizzard| {
            match blizzard {
                Blizzard::Up(x, y) => Blizzard::Up(x.clone(), ((y + round as usize - 1) % height) + 1),
                Blizzard::Down(x, y) => Blizzard::Down(x.clone(), ((y.clone() as i32 + (height - (round as usize % height)) as i32 - 1) % height as i32) as usize + 1),
                Blizzard::Left(x, y) => Blizzard::Left(((x.clone() as i32 + (width - (round as usize % width)) as i32 - 1) % width as i32) as usize + 1, y.clone()),
                Blizzard::Right(x, y) => Blizzard::Right(((x + round as usize - 1) % width) + 1, y.clone()),
            }
        }).collect()
}

// start and end are "out of bounds", so keep that in mind when calling
fn in_bounds(point: (usize, usize), width: usize, height: usize) -> bool {
    point.0 >= 1 && point.0 <= width && point.1 >= 1 && point.1 <= height
}

fn get_best_option(options: &HashSet<(usize, usize, u32)>, costs: &HashMap<(usize, usize, u32), u32>) -> (usize, usize, u32) {
    let mut best_option = (0, 0, 0);
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

fn will_intercept(point: (usize, usize), width: usize, height: usize, blizzards: &Vec<Blizzard>, round: u32) -> bool {
    blizzards.iter()
        .any(|blizzard| {
            match blizzard {
                Blizzard::Up(x, y) => (x.clone(), ((y + round as usize - 1) % height) + 1) == point,
                Blizzard::Down(x, y) => (x.clone(), ((y.clone() as i32 + (height - (round as usize % height)) as i32 - 1) % height as i32) as usize + 1) == point,
                Blizzard::Left(x, y) => (((x.clone() as i32 + (width - (round as usize % width)) as i32 - 1) % width as i32) as usize + 1, y.clone()) == point,
                Blizzard::Right(x, y) => (((x + round as usize - 1) % width) + 1, y.clone()) == point,
            }
        })
}

fn get_neighbors(node: (usize, usize), start: (usize, usize), end: (usize, usize), width: usize, height: usize, blizzards: &Vec<Blizzard>, round: u32) -> Vec<(usize, usize)> {
    let mut ret: Vec<(usize, usize)> = Vec::new();

    if !will_intercept(node, width, height, blizzards, round + 1) {
        ret.push(node.clone());
    } else if node == (1, height + 2) {
        println!("thought I would intercept the start position");
    }

    if in_bounds((node.0 + 1, node.1), width, height) && !will_intercept((node.0 + 1, node.1), width, height, blizzards, round + 1) {
        ret.push((node.0 + 1, node.1));
    }

    if in_bounds((node.0 - 1, node.1), width, height) && !will_intercept((node.0 - 1, node.1), width, height, blizzards, round + 1) {
        ret.push((node.0 - 1, node.1));
    }

    if (in_bounds((node.0, node.1 + 1), width, height) && !will_intercept((node.0, node.1 + 1), width, height, blizzards, round + 1)) || (node.0, node.1 + 1) == start {
        ret.push((node.0, node.1 + 1));
    }

    if node != end && ((in_bounds((node.0, node.1 - 1), width, height) && !will_intercept((node.0, node.1 - 1), width, height, blizzards, round + 1)) || (node.0, node.1 - 1) == end) {
        ret.push((node.0, node.1 - 1));
    }

    ret
}

fn search(start: (usize, usize), end: (usize, usize), width: usize, height: usize, blizzards: &Vec<Blizzard>) -> u32 {
    let mut open_set: HashSet<(usize, usize, u32)> = HashSet::new();
    let mut actual_costs: HashMap<(usize, usize, u32), u32> = HashMap::new();
    open_set.insert((start.0, start.1, 0));
    actual_costs.insert((start.0, start.1, 0), 0);

    while !open_set.is_empty() {
        let current = get_best_option(&open_set, &actual_costs);
        open_set.remove(&current);
        if (current.0, current.1) == end {
            return current.2;
        }

        for neighbor in get_neighbors((current.0, current.1), start, end, width, height, blizzards, current.2) {
            let neighbor = (neighbor.0, neighbor.1, current.2 + 1);
            let new_actual_cost = actual_costs.get(&current).unwrap() + 1;
            if new_actual_cost < actual_costs.get(&neighbor).cloned().unwrap_or(u32::MAX) {
                actual_costs.insert(neighbor, new_actual_cost);
                open_set.insert(neighbor);
            }
        }


    }

    panic!("end not found");
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut blizzards: Vec<Blizzard> = Vec::new();
    let height = input.lines().count() - 2;
    let mut width = 0;
    for (i, line) in input.lines().enumerate() {
        width = line.len() - 2;
        for (j, char) in line.chars().enumerate() {
            match char {
                '^' => blizzards.push(Blizzard::Up(j, (height + 1 - i))),
                'v' => blizzards.push(Blizzard::Down(j, (height + 1 - i))),
                '<' => blizzards.push(Blizzard::Left(j, (height + 1 - i))),
                '>' => blizzards.push(Blizzard::Right(j ,(height + 1 - i))),
                _   => (),
            }
        }
    }

    println!("width = {}, height = {}", width, height);
    println!("{:?}", blizzards);
    println!("{:?}", get_landscape(&blizzards, 0, width, height));
    println!("{:?}", get_landscape(&blizzards, 1, width, height));

    // width and height are for blizzard area, and are two less than the total
    let start = (1, height + 1);
    let end = (width, 0);

    Some(search(start, end, width, height, &blizzards))
}

fn get_best_forgetful_option(options: &HashSet<(usize, usize, u32, bool, bool)>, costs: &HashMap<(usize, usize, u32, bool, bool), u32>) -> (usize, usize, u32, bool, bool) {
    let mut best_option = (0, 0, 0, false, false);
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

fn forgetful_search(start: (usize, usize), end: (usize, usize), width: usize, height: usize, blizzards: &Vec<Blizzard>) -> u32 {
    let mut open_set: HashSet<(usize, usize, u32, bool, bool)> = HashSet::new();
    let mut actual_costs: HashMap<(usize, usize, u32, bool, bool), u32> = HashMap::new();
    open_set.insert((start.0, start.1, 0, false, false));
    actual_costs.insert((start.0, start.1, 0, false, false), 0);
    let mut global_first_finish = false;
    let mut global_got_keys = false;

    while !open_set.is_empty() {
        let current = get_best_forgetful_option(&open_set, &actual_costs);
        open_set.remove(&current);
        if current.3 == false && global_first_finish == true {
            continue;
        }
        if current.4 == false && global_got_keys == true {
            continue;
        }
        if (current.0, current.1) == end && current.3 == true && current.4 == true {
            return current.2;
        }

        for neighbor in get_neighbors((current.0, current.1), start, end, width, height, blizzards, current.2) {
            let mut neighbor = (neighbor.0, neighbor.1, current.2 + 1, current.3, current.4);
            if (neighbor.0, neighbor.1) == end {
                neighbor.3 = true;
                global_first_finish = true;
                println!("got first finish");
            }

            if (neighbor.0, neighbor.1) == start && current.3 == true {
                neighbor.4 = true;
                global_got_keys = true;
                println!("got keys");
            }

            let new_actual_cost = actual_costs.get(&current).unwrap() + 1;
            if new_actual_cost < actual_costs.get(&neighbor).cloned().unwrap_or(u32::MAX) {
                actual_costs.insert(neighbor, new_actual_cost);
                open_set.insert(neighbor);
            }
        }


    }

    panic!("end not found");
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut blizzards: Vec<Blizzard> = Vec::new();
    let height = input.lines().count() - 2;
    let mut width = 0;
    for (i, line) in input.lines().enumerate() {
        width = line.len() - 2;
        for (j, char) in line.chars().enumerate() {
            match char {
                '^' => blizzards.push(Blizzard::Up(j, (height + 1 - i))),
                'v' => blizzards.push(Blizzard::Down(j, (height + 1 - i))),
                '<' => blizzards.push(Blizzard::Left(j, (height + 1 - i))),
                '>' => blizzards.push(Blizzard::Right(j ,(height + 1 - i))),
                _   => (),
            }
        }
    }

    println!("width = {}, height = {}", width, height);
    println!("{:?}", blizzards);
    println!("{:?}", get_landscape(&blizzards, 0, width, height));
    println!("{:?}", get_landscape(&blizzards, 1, width, height));

    // width and height are for blizzard area, and are two less than the total
    let start = (1, height + 1);
    let end = (width, 0);

    Some(forgetful_search(start, end, width, height, &blizzards))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 24);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_two(&input), None);
    }
}
