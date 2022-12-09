use std::collections::HashSet;

pub fn check_num_unique(packet: &Vec<&char>, num: u32) -> bool {
    let unique: HashSet<_> = HashSet::from_iter(packet.iter());
    (unique.len() as u32) == num
}

pub fn part_one(input: &str) -> Option<u32> {
    let chars: Vec<_> = input.chars().collect();
    let mut current_packet: Vec<&char> = Vec::new();
    let start_packet_index = 0;

    for i in 0..chars.len() {
        current_packet.push(&chars[i]);
        if check_num_unique(&current_packet, 4) == true {
            return Some((i as u32) + 1);
        }

        if current_packet.len() == 4 {
            current_packet.remove(0);
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let chars: Vec<_> = input.chars().collect();
    let mut current_packet: Vec<&char> = Vec::new();
    let start_packet_index = 0;

    for i in 0..chars.len() {
        current_packet.push(&chars[i]);
        if check_num_unique(&current_packet, 14) == true {
            return Some((i as u32) + 1);
        }

        if current_packet.len() == 14 {
            current_packet.remove(0);
        }
    }

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
