use std::borrow::BorrowMut;
use std::collections::HashSet;

fn get_head_transform(direction: &str) -> (i32, i32) {
    match direction {
        "U" => (0, 1),
        "R" => (1, 0),
        "D" => (0, -1),
        "L" => (-1, 0),
        _ => panic!("unexpected direction")
    }
}

fn get_tail_transform(diff: (i32, i32)) -> (i32, i32) {
    if diff.0.abs() > 1 || diff.1.abs() > 1 {
        (diff.0.clamp(-1, 1), diff.1.clamp(-1, 1))
    } else {
        (0, 0)
    }
}

pub fn move_direction(direction: &str, knots: &mut Vec<(i32, i32)>, tail_locations: &mut HashSet<(i32, i32)>) -> () {
    let head_transform = get_head_transform(direction);
    knots[0].0 += head_transform.0;
    knots[0].1 += head_transform.1;

    let mut prev_knot = knots[0];
    for mut knot in &mut knots[1..] {
        let tail_transform = get_tail_transform((prev_knot.0 - knot.0, prev_knot.1 - knot.1));
        knot.0 += tail_transform.0;
        knot.1 += tail_transform.1;
        prev_knot = (knot.0, knot.1);
    }
    tail_locations.insert(knots[knots.len() - 1]);

    // println!("({}, {})", head_location.0, head_location.1);
}

fn process_move(current_move: &str, knots: &mut Vec<(i32, i32)>, tail_locations: &mut HashSet<(i32, i32)>) -> () {
    let instructions: Vec<_> = current_move.split(" ").collect();
    let direction = instructions[0];
    let magnitude = instructions[1].parse::<u32>().unwrap();
    for i in 0..magnitude {
        move_direction(direction, knots, tail_locations);
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut knots: Vec<(i32, i32)> = Vec::from([(0, 0), (0, 0)]);

    let mut tail_locations: HashSet<(i32, i32)> = HashSet::new();
    tail_locations.insert((0, 0));

    for line in input.lines() {
        process_move(line, &mut knots, &mut tail_locations);
    }

    // 6243
    Some(tail_locations.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut knots: Vec<(i32, i32)> = Vec::new();
    for i in 0..10 {
        println!("{}", i);
        knots.push((0, 0));
    }

    let mut tail_locations: HashSet<(i32, i32)> = HashSet::new();
    tail_locations.insert((0, 0));

    for line in input.lines() {
        process_move(line, &mut knots, &mut tail_locations);
    }

    // 6243
    Some(tail_locations.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }
}
