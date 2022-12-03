use std::collections::HashSet;

pub fn get_score(letter: &str) -> u8 {
    let char = letter.to_lowercase().as_bytes()[0];
    char - "a".as_bytes()[0] + 1 + if letter != letter.to_lowercase() {
        26
    } else {
        0
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total_score: u32 = 0;
    for line in input.lines() {
        let size = line.len();
        let first_compartment = &line[..(size / 2)];
        let second_compartment = &line[(size / 2)..];
        println!("{} | {}", first_compartment, second_compartment);
        let first_item_types: HashSet<char> = first_compartment.chars().collect();
        let second_item_types: HashSet<char> = second_compartment.chars().collect();

        let intersection: HashSet<_> = first_item_types.intersection(&second_item_types).collect();
        if intersection.len() != 1 {
            println!("Multiple overlaping types");
            return None
        }

        for letter in &intersection {
            let letter = letter.to_string();
            total_score += get_score(&letter) as u32;
        }
    }

    Some(total_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sacks: Vec<_> = input.lines().collect();
    let mut total_score: u32 = 0;
    for i in 0..(sacks.len() / 3) {
        let mut a: HashSet<char> = sacks[(3*i)].chars().collect();
        let mut b: HashSet<char> = sacks[(3*i)+1].chars().collect();
        let mut c: HashSet<char> = sacks[(3*i)+2].chars().collect();

        let mut badge_set: HashSet<char> = a.intersection(&b).cloned().collect();
        badge_set = badge_set.intersection(&c).cloned().collect();
        // let mut test: HashSet<char> = a.intersection(&c).collect();
        // let mut final: HashSet<&&char> = badge_set.intersection(&test).collect();

        if badge_set.len() != 1 {
            println!("Found more than one badge for group");
            return None
        }

        for letter in &badge_set {
            let letter = letter.to_string();
            total_score += get_score(&letter) as u32
        }
    }

    Some(total_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
