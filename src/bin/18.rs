use std::collections::HashSet;

#[derive(Debug)]
struct Bounds {
    x1: u32,
    x2: u32,
    y1: u32,
    y2: u32,
    z1: u32,
    z2: u32,
}


fn get_dist(p1: &(u32, u32, u32), p2: &(u32, u32 ,u32)) -> u32 {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1) + p1.2.abs_diff(p2.2)
}

fn add_line(line: &str, current_cubes: &mut HashSet<(u32, u32, u32)>, bounds: &mut Bounds, sa: &mut u32) -> () {
    let parts: Vec<&str> = line.split(",").collect();
    let x = parts[0].parse::<u32>().unwrap();
    let y = parts[1].parse::<u32>().unwrap();
    let z = parts[2].parse::<u32>().unwrap();

    bounds.x1 = bounds.x1.min(x);
    bounds.x2 = bounds.x2.max(x);
    bounds.y1 = bounds.y1.min(y);
    bounds.y2 = bounds.y2.max(y);
    bounds.z1 = bounds.z1.min(z);
    bounds.z2 = bounds.z2.max(z);

    let mut num_connected = 0;
    for cube in current_cubes.iter() {
        if get_dist(&(x, y, z), cube) == 1 {
            num_connected += 1;
        }
    }

    if num_connected <= 3 {
        *sa += 6 - (2 * num_connected);
    } else {
        *sa -= (2 * (num_connected - 3));
    }

    current_cubes.insert((x, y, z));
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sa = 0;
    let mut current_cubes: HashSet<(u32, u32, u32)> = HashSet::new();
    let mut bounds: Bounds = Bounds { x1: 10, x2: 10, y1: 10, y2: 10, z1: 10, z2: 10 };
    for line in input.lines() {
        add_line(line, &mut current_cubes, &mut bounds, &mut sa)
    }

    Some(sa)
}


fn get_neighbors(point: &(u32, u32, u32), current_cubes: &HashSet<(u32, u32, u32)>, visited: &HashSet<(u32, u32, u32)>) -> Vec<(u32, u32, u32)> {
    let mut ret: Vec<(u32, u32, u32)> = Vec::new();
    let (x, y, z) = point.clone();
    let check = |point: (u32, u32, u32)| !current_cubes.contains(&point) && !visited.contains(&point);

    if check((x + 1, y, z)) {
        ret.push((x + 1, y, z));
    }
    if check((x - 1, y, z)) {
        ret.push((x - 1, y, z));
    }
    if check((x, y + 1, z)) {
        ret.push((x, y + 1, z));
    }
    if check((x, y - 1, z)) {
        ret.push((x, y - 1, z));
    }
    if check((x, y, z + 1)) {
        ret.push((x, y, z + 1));
    }
    if check((x, y, z - 1)) {
        ret.push((x, y, z - 1));
    }

    ret
}

fn on_boundary(point: &(u32, u32, u32), bounds: &Bounds) -> bool {
    point.0 == bounds.x1 || point.0 == bounds.x2 || point.1 == bounds.y1 || point.1 == bounds.y2 || point.2 == bounds.z1 || point.2 == bounds.z2
}

fn is_surrounded(point: &(u32, u32, u32), current_cubes: &HashSet<(u32, u32, u32)>, bounds: &Bounds) -> bool {
    let mut visited: HashSet<(u32, u32, u32)> = HashSet::new();
    let mut possible: Vec<(u32, u32, u32)> = Vec::new();
    possible.push(point.clone());

    while !possible.is_empty() {
        let current = possible.pop().unwrap();
        visited.insert(current);
        if on_boundary(&current, bounds) {
            return false;
        }
        let neighbors = get_neighbors(&current, current_cubes, &visited);
        for neighbor in neighbors {
            possible.push(neighbor);
        }
    }


    true
}

fn add_point(point: (u32, u32, u32), current_cubes: &mut HashSet<(u32, u32, u32)>, sa: &mut u32) -> () {
    let (x, y, z) = point;

    println!("adding point {:?}", point);

    let mut num_connected = 0;
    for cube in current_cubes.iter() {
        if get_dist(&(x, y, z), cube) == 1 {
            num_connected += 1;
        }
    }

    println!("found {} connected", num_connected);
    println!("current sa is {}", sa);


    if num_connected <= 3 {
        *sa += 6 - (2 * num_connected);
    } else {
        *sa -= (2 * (num_connected - 3));
    }

    current_cubes.insert((x, y, z));
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sa = 0;
    let mut current_cubes: HashSet<(u32, u32, u32)> = HashSet::new();
    let mut bounds: Bounds = Bounds { x1: 10, x2: 10, y1: 10, y2: 10, z1: 10, z2: 10 };
    for line in input.lines() {
        add_line(line, &mut current_cubes, &mut bounds, &mut sa)
    }

    for x in bounds.x1..(bounds.x2 + 1) {
        for y in bounds.y1..(bounds.y2 + 1) {
            for z in bounds.z1..(bounds.z2 + 1) {
                if !current_cubes.contains(&(x, y, z)) && is_surrounded(&(x, y, z), &current_cubes, &bounds) {
                    add_point((x, y, z), &mut current_cubes, &mut sa);
                }
            }
        }
    }

    println!("{:?}", bounds);

    Some(sa)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), None);
    }
}
