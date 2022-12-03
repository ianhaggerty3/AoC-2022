use std::num::Wrapping;

pub fn part_one(input: &str) -> Option<u32> {
    let mut points_total = 0;
    for line in input.lines() {
        let bytes = line.as_bytes();
        let first: i32 = bytes[0] as i32;
        let second: i32 = (bytes[2] - 23) as i32;
        let second_char = line.chars().nth(2).unwrap();
        let diff = second - first;
        let mut points_round = 0;

        points_round += if diff == 1 || diff == -2 {
            6
        } else if diff == 2 || diff == -1 {
            0
        } else {
            3
        };

        points_round += if second_char == 'Z' {
            3
        } else if second_char == 'Y' {
            2
        } else {
            1
        };

        println!("{}: {}, {}, {}", line, points_round, first, second);

        points_total += points_round
    }

    Some(points_total)
}

pub fn get_score(choice: char) -> u32 {
    if choice == 'A' {
        1
    } else if choice == 'B' {
        2
    } else {
        3
    }
}

pub fn get_winning(choice: char) -> char {
    if choice == 'A' {
        'B'
    } else if choice == 'B' {
        'C'
    } else {
        'A'
    }
}

pub fn get_losing(choice: char) -> char {
    if choice == 'A' {
        'C'
    } else if choice == 'B' {
        'A'
    } else {
        'B'
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut points_total = 0;
    for line in input.lines() {
        let bytes = line.as_bytes();
        let first: i32 = bytes[0] as i32;
        let first_char = line.chars().nth(0).unwrap();
        let second: i32 = (bytes[2] - 23) as i32;
        let second_char = line.chars().nth(2).unwrap();
        let mut points_round = 0;

        points_round += if second_char == 'X' {
            0 + get_score(get_losing(first_char))
        } else if second_char == 'Y' {
            3 + get_score(first_char)
        } else {
            6 + get_score(get_winning(first_char))
        };



        points_total += points_round;
    }

    Some(points_total)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
