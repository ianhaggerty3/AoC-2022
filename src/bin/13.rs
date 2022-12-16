use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::iter::zip;

#[derive(Debug, Clone)]
enum Item {
    list(Vec<Item>),
    integer(u32),
}

// I'm not proud, but it will (probably) get  me unstuck
fn get_num(packet: &Vec<char>, current_char: &mut usize) -> u32 {
    let old_char: usize = current_char.clone();
    while packet[*current_char].is_numeric() {
        *current_char += 1;
    }

    let num: String = packet[old_char..*current_char].iter().collect();
    // println!("parsing {}", num);
    // println!("parsed {}", num.parse::<u32>().unwrap());
    num.parse::<u32>().unwrap()
}

fn parse_packet(packet: &Vec<char>, mut current_char: &mut usize) -> Option<Item> {
    while packet[*current_char] == ',' {
        *current_char += 1;
    }

    if packet[*current_char] == '[' {
        let mut ret: Vec<Item> = Vec::new();
        *current_char += 1;
        while let Some(inner_packet) = parse_packet(packet, current_char) {
            ret.push(inner_packet);
        }
        Some(Item::list(ret))
    } else if packet[*current_char] == ']' {
        *current_char += 1;
        None
    } else {
        let ret = get_num(packet, current_char);

        Some(Item::integer(ret))
    }
}

fn compare_item(a: Item, b: Item) -> i32 {
    if let Item::integer(a) = a {
        if let Item::integer(b) = b {
            return (a as i32) - (b as i32);
        } else if let Item::list(b_packet) = &b {
            let expanded: Item = Item::list(Vec::from([Item::integer(a)]));
            return compare_item(expanded, b);
        }
    } else if let Item::list(a_list) = &a {
        if let Item::integer(b) = b {
            let expanded: Item = Item::list(Vec::from([Item::integer(b)]));
            return compare_item(a, expanded);
        } else if let Item::list(b) = b {
            let mut i = 0;
            while i < a_list.len() {
                if i >= b.len() {
                    return 1; // out of order
                }

                let result = compare_item(a_list[i].clone(), b[i].clone());
                if result != 0 {
                    return result; // out of order
                }
                i += 1;
            }
            if i < b.len() {
                return -1; // in order
            }
        }
    }

    0
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut pairs: Vec<(Item, Item)> = Vec::new();
    let lines: Vec<&str> = input.lines().collect();
    let mut lines_iter = lines.iter();
    let firsts = lines.iter().step_by(3);
    lines_iter.next();

    let seconds = lines_iter.step_by(3);
    for (first, second) in zip(firsts, seconds) {
        println!("{}", first);
        println!("{}", second);
        let mut first: Vec<char> = first.chars().collect();
        let mut first_index = 0;
        let mut second: Vec<char> = second.chars().collect();
        let mut second_index = 0;

        pairs.push((
            parse_packet(&mut first, &mut first_index).unwrap(),
            parse_packet(&mut second, &mut second_index).unwrap(),
        ));
    }
    let mut i = 1;
    let mut sum = 0;
    for pair in pairs {
        // println!("first = {:?}", pair.0);
        // println!("second = {:?}", pair.1);

        let result = compare_item(pair.0, pair.1);
        if result > 0 {
            println!("out of order!");
        } else if result == 0 {
            println!("huh");
        } else {
            sum += i;
            println!("in order!");
        }
        i += 1;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut list: Vec<Item> = Vec::new();
    let lines: Vec<&str> = input.lines().collect();
    let mut lines_iter = lines.iter();
    let firsts = lines.iter().step_by(3);
    lines_iter.next();

    let seconds = lines_iter.step_by(3);
    for (first, second) in zip(firsts, seconds) {
        println!("{}", first);
        println!("{}", second);
        let mut first: Vec<char> = first.chars().collect();
        let mut first_index = 0;
        let mut second: Vec<char> = second.chars().collect();
        let mut second_index = 0;

        list.push(parse_packet(&mut first, &mut first_index).unwrap());
        list.push(parse_packet(&mut second, &mut second_index).unwrap());
    }

    let first_divider = Item::list(Vec::from([Item::list(Vec::from([Item::integer(2)]))]));
    let second_divider = Item::list(Vec::from([Item::list(Vec::from([Item::integer(6)]))]));
    list.push(first_divider.clone());
    list.push(second_divider.clone());

    list.sort_by(|a, b| {
        let result = compare_item(a.clone(), b.clone());
        if result < 0 {
            return Ordering::Less;
        } else if result > 0 {
            return Ordering::Greater;
        }

        Ordering::Equal
    });
    // println!("{:?}", list);

    let mut i = 1;
    let mut first_loc = 0;
    let mut second_loc = 0;
    // 90902 is too high
    for item in &list {
        let result = compare_item(first_divider.clone(), item.clone());
        if result == 0 {
            println!("found match at {}!", i);
            first_loc = i;
        }

        let result = compare_item(second_divider.clone(), item.clone());
        if result == 0 {
            println!("found match at {}!", i);
            second_loc = i;
        }
        i += 1;
    }
    Some(first_loc * second_loc)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), None);
    }
}
