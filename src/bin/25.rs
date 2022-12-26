use std::collections::HashMap;

fn val_map(val: char) -> i64 {
    match val {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => panic!("unexpected input value")
    }
}

fn rev_val_map(val: i64) -> char {
    match val {
        2 => '2',
        1 => '1',
        0 => '0',
        4 => '-',
        3 => '=',
        _ => panic!("unexpected input value")
    }

}

fn fu_to_int(num: &str) -> i64 {
    let nums: Vec<i64> = num.chars().map(|char| val_map(char)).collect();
    let mut sum = 0;
    let mut place = 1;
    for num in nums.iter().rev() {
        sum += num * place;
        place *= 5;
    }

    sum
}

fn int_to_fu(num: i64) -> String {
    let mut ret: Vec<char> = Vec::new();
    let mut current = num;

    while current > 0 {
        ret.insert(0, rev_val_map(current % 5));
        let remainder = (current % 5) / 3;
        current = current / 5;
        current = current + remainder;
    }

    ret.iter().collect()
}

pub fn part_one(input: &str) -> Option<String> {
    let mut sum = 0;
    for line in input.lines() {
        sum += fu_to_int(line);
    }

    Some(int_to_fu(sum))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 25);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_one(&input), Some("2=-1=0".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_two(&input), None);
    }
}
