use std::cmp;
use std::thread::current;

fn get_max_values(v: &mut Vec<u32>, n: usize) -> u32 {
    v.sort();
    v.reverse();

    let mut sum: u32 = 0;
    for i in 0..n {
        sum += v.get(i).unwrap().clone();
    }

    sum
}

pub fn part_one(input: &str) -> Option<u32> {
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

    Some(get_max_values(&mut v, 1))
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

    Some(get_max_values(&mut v, 3))
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
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
