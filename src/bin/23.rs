use std::collections::HashSet;

fn get_surrounding(location: &(i32, i32)) -> Vec<(i32, i32)> {
    let (x, y) = location;
    Vec::from([
        (x + 1, y + 1),
        (x + 1, y.clone()),
        (x + 1, y - 1),
        (x - 1, y + 1),
        (x - 1, y.clone()),
        (x - 1, y - 1),
        (x.clone(), y + 1),
        (x.clone(), y - 1),
    ])
}

fn get_move(elf: &(i32, i32), neighbors: Vec<(i32, i32)>, round: u32) -> (i32, i32) {
    for i in round..(round + 4) {
        let i = i % 4;
        if i == 0 {
            if neighbors.iter()
                .map(|loc| loc.1 <= elf.1)
                .fold(true, |a, b|a && b) {
                return (elf.0, elf.1 + 1)
            }
        } else if i == 1 {
            if neighbors.iter()
                .map(|loc| loc.1 >= elf.1)
                .fold(true, |a, b|a && b) {
                return (elf.0, elf.1 - 1)
            }
        } else if i == 2 {
            if neighbors.iter()
                .map(|loc| loc.0 >= elf.0)
                .fold(true, |a, b|a && b) {
                return (elf.0 - 1, elf.1)
            }
        } else {
            if neighbors.iter()
                .map(|loc| loc.0 <= elf.0)
                .fold(true, |a, b|a && b) {
                return (elf.0 + 1, elf.1)
            }
        }
    }

    elf.clone()
}

fn get_decision(elf: &(i32, i32), elves: &Vec<(i32, i32)>, round: u32, moved: &mut bool) -> (i32, i32) {
    let surrounding = get_surrounding(elf);
    let neighbors: Vec<(i32, i32)> = surrounding.iter()
        .filter(|loc| elves.contains(loc)).map(|item| item.clone()).collect();

    if neighbors.len() == 0 {
        return elf.clone();
    }

    *moved = true;

    get_move(elf, neighbors, round)
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut elves: Vec<(i32, i32)> = Vec::new();
    for (i, line) in input.lines().enumerate() {
        for (j, val) in line.chars().enumerate() {
            if val == '#' {
                elves.push((j as i32, (line.len() - i) as i32));
            }
        }
    }

    for i in 0..10 {
        let mut new_elves: Vec<(i32, i32)> = Vec::new();
        let mut moved = false;
        let moves: Vec<(i32, i32)> = elves.iter().map(|elf|get_decision(&elf, &elves, i, &mut moved)).collect();
        for (i, current_move) in moves.iter().enumerate() {
            if moves.iter().filter(|other_move|current_move.0 == other_move.0 && current_move.1 == other_move.1).count() == 1 {
                new_elves.push(current_move.clone());
            } else {
                new_elves.push(elves[i]);
            }
        }
        elves = new_elves;
    }

    let min_x = elves.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let max_x = elves.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let min_y = elves.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let max_y = elves.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;


    println!("{:?}", elves);

    Some(((max_x - min_x + 1) * (max_y - min_y + 1)) - elves.len() as i32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut elves: Vec<(i32, i32)> = Vec::new();
    for (i, line) in input.lines().enumerate() {
        for (j, val) in line.chars().enumerate() {
            if val == '#' {
                elves.push((j as i32, (line.len() - i) as i32));
            }
        }
    }


    let mut i = 0;
    loop {
        let mut moved = false;
        let mut new_elves: Vec<(i32, i32)> = Vec::new();
        let moves: Vec<(i32, i32)> = elves.iter().map(|elf|get_decision(&elf, &elves, i, &mut moved)).collect();
        i += 1;


        if !moved {
            break;
        }
        for (i, current_move) in moves.iter().enumerate() {
            if moves.iter().filter(|other_move|current_move.0 == other_move.0 && current_move.1 == other_move.1).count() == 1 {
                new_elves.push(current_move.clone());
            } else {
                new_elves.push(elves[i]);
            }
        }


        elves = new_elves;
    }

    Some(i)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 23);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_two(&input), None);
    }
}
