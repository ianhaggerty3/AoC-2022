use std::cmp::max;
use std::collections::HashMap;

#[derive(Clone, PartialEq)]
enum Move {
    left,
    right,
}

fn  get_starting_pos(max_height: u64) -> (u64, u64) {
    (2, max_height + 4)
}

fn check_out_of_bounds(piece: &Vec<(u64, u64)>) -> bool {
    piece.iter().any(|location| { location.0 < 0 || location.0 > 6 })
}

fn check_intersect(piece: &Vec<(u64, u64)>, map: &Vec<(u64, u64)>) -> bool {
    let mut ret= false;
    for loc in piece {
        if map.contains(loc) {
            ret = true;
        }
    }

    ret
}

fn sim_push(piece: Vec<(u64, u64)>, action: &Move, map: &Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let proposed_location: Vec<(u64, u64)> = piece.iter().map(|location| {
        let dir: i64 = if action.clone() == Move::left {
            -1
        } else {
            1
        };
        ((location.0 as i64 + dir) as u64, location.1)
    }).collect();

    if check_intersect(&proposed_location, map) || check_out_of_bounds(&proposed_location) {
        piece
    } else {
        proposed_location
    }
}

fn sim_fall(piece: Vec<(u64, u64)>, map: &Vec<(u64, u64)>, settled: &mut bool) -> Vec<(u64, u64)> {
    let proposed_location: Vec<(u64, u64)> = piece.iter().map(|location| {
        (location.0, location.1 - 1)
    }).collect();

    if check_intersect(&proposed_location, map) {
        *settled = true;
        piece
    } else {
        proposed_location
    }
}

fn add_piece(piece: Vec<(u64, u64)>, map: &mut Vec<(u64, u64)>, heights: &mut Vec<u64>, max_height: &mut u64) {
    for location in piece {
        map.insert(0, location);
        *max_height = max_height.clone().max(location.1);
        heights[location.0 as usize] = heights[location.0 as usize].max(location.1);
    }
}

fn move_piece_by(piece: Vec<(u64, u64)>, dir: (u64, u64)) -> Vec<(u64, u64)> {
    piece.iter().map(|location| (location.0 + dir.0, location.1 + dir.1)).collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut map: Vec<(u64, u64)> = Vec::from([(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0)]);
    let mut shapes: Vec<Vec<(u64, u64)>> = Vec::new();
    shapes.push(Vec::from([(0, 0), (1, 0), (2, 0), (3, 0)]));
    shapes.push(Vec::from([(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]));
    shapes.push(Vec::from([(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]));
    shapes.push(Vec::from([(0, 0), (0, 1), (0, 2), (0, 3)]));
    shapes.push(Vec::from([(0, 0), (1, 0), (0, 1), (1, 1)]));

    let moves: Vec<Move> = input.trim().chars().map(|char| {
        let mut ret: Move = Move::left;
        if char == '<' {
            ret = Move::left
        } else {
            ret = Move::right
        }

        ret
    }).collect();

    let moves_length = moves.len();
    let mut moves = moves.iter().cycle();
    let mut heights = Vec::from([0, 0, 0, 0, 0, 0, 0]);


    let mut seen_map: HashMap<(usize, usize, (u64, u64, u64, u64, u64, u64, u64)), (u64, u64)> = HashMap::new();

    let mut max_height = 0;
    let mut move_index = 0;
    let mut shape_index: usize = 0;
    let mut absolute_index: u64 = 0;
    let mut last_min: u64 = 0;
    let mut historic_height: Vec<u64> = Vec::new();

    for shape in shapes.iter().cycle().take(2022) {
        let mut settled = false;
        let mut current_shape = shape.clone();
        current_shape = move_piece_by(current_shape, get_starting_pos(max_height));

        while !settled {
            current_shape = sim_push(current_shape, moves.next().unwrap(), &map);
            current_shape = sim_fall(current_shape, &map, &mut settled);
        }

        add_piece(current_shape, &mut map, &mut heights, &mut max_height);
        let rel_heights = get_rel_heights(&heights);
        if seen_map.contains_key(&(move_index, shape_index, rel_heights)) {
            let old_index = seen_map[&(move_index, shape_index, rel_heights)].0;
            let old_height = seen_map[&(move_index, shape_index, rel_heights)].1;
            println!("found a cycle from index = {} to index = {}", old_index, absolute_index);
            println!("found a cycle from height = {} to height = {}", old_height, max_height);
            let target = 2022 - 1;
            let index_diff = absolute_index - old_index;
            let height_diff = max_height - old_height;
            let remaining = target - absolute_index;
            let cycles_remaining = remaining / index_diff;
            let cycles_height = cycles_remaining * height_diff;
            let remainder_index = old_index + (remaining % index_diff);
            let remainder_height = historic_height[remainder_index as usize] - old_height;
            println!("remainder len = {}", remaining % index_diff);
            println!("adding {} height from remaining cycles and a {} height remainder", cycles_height, remainder_height);
            
            return Some(max_height + cycles_height + remainder_height);
        }
        seen_map.insert((move_index, shape_index, rel_heights), (absolute_index, max_height));
        move_index = (move_index + 1) % moves_length;
        shape_index = (shape_index + 1) % 5;
        absolute_index += 1;

        if absolute_index % 100 == 0 {
            map = clean_map(map, &heights, &mut last_min);
        }
        historic_height.push(max_height);
    }

    Some(map.iter().max_by(|a, b| { a.1.cmp(&b.1) }).unwrap().1)
}

fn get_height(map: &Vec<(u64, u64)>, index: usize) -> u64 {
    map.iter().max_by(|a, b| { a.1.cmp(&b.1) }).unwrap().1
}

fn get_rel_heights(heights: &Vec<u64>) -> (u64, u64, u64, u64, u64, u64, u64) {
    let m = heights.iter().max().unwrap();
    (m - heights[0], m - heights[1], m - heights[2], m - heights[3], m - heights[4], m - heights[5], m - heights[6])
}

fn clean_map(map: Vec<(u64, u64)>, heights: &Vec<u64>, last_min: &mut u64) -> Vec<(u64, u64)> {
    let lowest_height = heights.iter().min().unwrap();
    for i in (last_min.clone()..lowest_height.clone()).rev() {
        let mut is_bar = true;
        for j in 0..7 {
            if !map.contains(&(j, i.clone())) {
                is_bar = false;
                break;
            }
        }
        if is_bar {
            // println!("cleaning because I found a bar at y = {}", i);
            *last_min = i.clone();
            return map.iter().filter(|location| location.1 >= i.clone()).map(|item| item.clone()).collect()
        }

    }
    map
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map: Vec<(u64, u64)> = Vec::from([(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0)]);
    let mut shapes: Vec<Vec<(u64, u64)>> = Vec::new();
    shapes.push(Vec::from([(0, 0), (1, 0), (2, 0), (3, 0)]));
    shapes.push(Vec::from([(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]));
    shapes.push(Vec::from([(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]));
    shapes.push(Vec::from([(0, 0), (0, 1), (0, 2), (0, 3)]));
    shapes.push(Vec::from([(0, 0), (1, 0), (0, 1), (1, 1)]));

    let moves: Vec<Move> = input.trim().chars().map(|char| {
        let mut ret: Move = Move::left;
        if char == '<' {
            ret = Move::left
        } else {
            ret = Move::right
        }

        ret
    }).collect();

    let moves_length = moves.len();
    let mut moves = moves.iter().cycle();
    let mut heights = Vec::from([0, 0, 0, 0, 0, 0, 0]);


    let mut seen_map: HashMap<(usize, usize, (u64, u64, u64, u64, u64, u64, u64)), (u64, u64)> = HashMap::new();

    let mut max_height = 0;
    let mut move_index = 0;
    let mut shape_index: usize = 0;
    let mut absolute_index: u64 = 0;
    let mut last_min: u64 = 0;
    let mut historic_height: Vec<u64> = Vec::new();

    for shape in shapes.iter().cycle().take(1000000000000) {
        let mut settled = false;
        let mut current_shape = shape.clone();
        current_shape = move_piece_by(current_shape, get_starting_pos(max_height));

        while !settled {
            current_shape = sim_push(current_shape, moves.next().unwrap(), &map);
            current_shape = sim_fall(current_shape, &map, &mut settled);
        }

        add_piece(current_shape, &mut map, &mut heights, &mut max_height);
        let rel_heights = get_rel_heights(&heights);
        if seen_map.contains_key(&(move_index, shape_index, rel_heights)) {
            let old_index = seen_map[&(move_index, shape_index, rel_heights)].0;
            let old_height = seen_map[&(move_index, shape_index, rel_heights)].1;
            println!("found a cycle from index = {} to index = {}", old_index, absolute_index);
            println!("found a cycle from height = {} to height = {}", old_height, max_height);
            let target = 1_000_000_000_000 - 1;
            let index_diff = absolute_index - old_index;
            let height_diff = max_height - old_height;
            let remaining = target - absolute_index;
            let cycles_remaining = remaining / index_diff;
            let cycles_height = cycles_remaining * height_diff;
            let remainder_index = old_index + (remaining % index_diff);
            let remainder_height = historic_height[remainder_index as usize] - old_height;
            println!("remainder len = {}", remaining % index_diff);
            println!("adding {} height from remaining cycles and a {} height remainder", cycles_height, remainder_height);


            return Some(max_height + cycles_height + remainder_height);
        }
        seen_map.insert((move_index, shape_index, rel_heights), (absolute_index, max_height));
        move_index = (move_index + 1) % moves_length;
        shape_index = (shape_index + 1) % 5;
        absolute_index += 1;

        if absolute_index % 100 == 0 {
            map = clean_map(map, &heights, &mut last_min);
        }
        historic_height.push(max_height);
    }

    Some(map.iter().max_by(|a, b| { a.1.cmp(&b.1) }).unwrap().1)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), None);
    }
}
