use std::cmp;
use std::thread::current;

pub fn part_one(input: &str) -> Option<u32> {
    let mut maxCalories= 0;
    let mut currentCalories = 0;

    for line in input.lines() {
        if line.is_empty() {
            maxCalories = cmp::max(maxCalories, currentCalories);
            currentCalories = 0;
            continue;
        }
        currentCalories += line.parse::<u32>().unwrap();
    }

    Some(cmp::max(maxCalories, currentCalories))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut v: Vec<u32> = Vec::new();

    let mut currentCalories = 0;
    for line in input.lines() {
        if line.is_empty() {
            v.push(currentCalories);
            currentCalories = 0;
            continue;
        }
        currentCalories += line.parse::<u32>().unwrap();
    }

    v.sort();
    v.reverse();
    Some(v.get(0).unwrap() + v.get(1).unwrap() + v.get(2).unwrap())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
