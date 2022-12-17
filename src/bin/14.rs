use std::collections::HashSet;

#[derive(Debug)]
struct Bounds {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}

fn get_pair(item: &str) -> (i32, i32) {
    let parts: Vec<&str> = item.split(",").collect();
    (parts[0].parse::<i32>().unwrap(), parts[1].parse::<i32>().unwrap())
}

fn update_bounds(x: i32, y: i32, bounds: &mut Bounds) {
    bounds.x1 = bounds.x1.min(x);
    bounds.x2 = bounds.x2.max(x);
    bounds.y1 = bounds.y1.min(y);
    bounds.y2 = bounds.y2.max(y);
}

fn parse_line(line: &str, full_spaces: &mut HashSet<(i32, i32)>, bounds: &mut Bounds) {
    let line: Vec<&str> = line.split(" -> ").collect();
    let mut first = line[0];
    for second in &line[1..] {
        let (x1, y1) = get_pair(first);
        let (x2, y2) = get_pair(second);
        for x in x1.min(x2)..(x1.max(x2) + 1) {
            for y in y1.min(y2)..(y1.max(y2) + 1) {
                full_spaces.insert((x, y));
                update_bounds(x, y, bounds);
            }
        }

        first = second;
    }
}

fn is_out_of_bounds(sand: (i32, i32), bounds: &Bounds) -> bool {
    // y1 doesn't matter because sand falls up
    sand.0 < bounds.x1 || sand.0 > bounds.x2 || sand.1 > bounds.y2
}

fn sim_sand(full_spaces: &mut HashSet<(i32, i32)>, bounds: &Bounds) -> u32 {
    let starting_point = (500, 0);
    let mut out_of_bounds = false;
    let mut total = 0;
    while !out_of_bounds {
        let mut sand = starting_point;
        let mut settled = false;
        while !settled {
            if full_spaces.contains(&(sand.0, sand.1 + 1)) {
                if !full_spaces.contains(&(sand.0 - 1, sand.1 + 1)) {
                    sand.0 -= 1;
                    sand.1 += 1;
                } else if !full_spaces.contains(&(sand.0 + 1, sand.1 + 1)) {
                    sand.0 += 1;
                    sand.1 += 1;
                } else {
                    full_spaces.insert((sand.0, sand.1));
                    settled = true;
                }
            } else {
                sand.1 += 1;
            }

            if is_out_of_bounds(sand, bounds) {
                out_of_bounds = true;
                break;
            }
        }
        total += 1;
    }

    total - 1
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut full_spaces: HashSet<(i32, i32)> = HashSet::new();
    let mut bounds: Bounds = Bounds { x1: 500, x2: 500, y1: 100, y2: 100 };
    for line in input.lines() {
        parse_line(line, &mut full_spaces, &mut bounds);
    }
    println!("{:?}", bounds);

    Some(sim_sand(&mut full_spaces, &bounds))
}

fn add_floor(full_spaces: &mut HashSet<(i32, i32)>, bounds: &Bounds) {
    let relevant_y = bounds.y2 + 2;

    for x in -(relevant_y + 5)..(relevant_y + 5) {
        full_spaces.insert((500 + x, relevant_y));
    }
}

fn sim_sand_blocked(full_spaces: &mut HashSet<(i32, i32)>) -> u32 {
    let starting_point = (500, 0);
    let mut total = 0;
    while !full_spaces.contains(&starting_point) {
        let mut sand = starting_point;
        let mut settled = false;
        while !settled {
            if full_spaces.contains(&(sand.0, sand.1 + 1)) {
                if !full_spaces.contains(&(sand.0 - 1, sand.1 + 1)) {
                    sand.0 -= 1;
                    sand.1 += 1;
                } else if !full_spaces.contains(&(sand.0 + 1, sand.1 + 1)) {
                    sand.0 += 1;
                    sand.1 += 1;
                } else {
                    full_spaces.insert((sand.0, sand.1));
                    settled = true;
                }
            } else {
                sand.1 += 1;
            }
        }
        total += 1;
    }

    total
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut full_spaces: HashSet<(i32, i32)> = HashSet::new();
    let mut bounds: Bounds = Bounds { x1: 500, x2: 500, y1: 100, y2: 100 };
    for line in input.lines() {
        parse_line(line, &mut full_spaces, &mut bounds);
    }
    add_floor(&mut full_spaces, &bounds);
    Some(sim_sand_blocked(&mut full_spaces))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), None);
    }
}
