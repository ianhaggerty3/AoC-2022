use std::collections::HashMap;

fn mod_add(a: usize, b: usize, orig_list: &Vec<i32>) -> usize {
    (a + b) % orig_list.len()
}

fn mix(start: usize, orig_list: &Vec<i32>, orig_to_new: &mut HashMap<usize, usize>, new_to_orig: &mut HashMap<usize, usize>) {
    let cur = orig_list[start];
    let cur = cur % orig_list.len() as i32;
    for i in 1..cur {
         let tmp = new_to_orig[&orig_to_new[&start]];
         new_to_orig.insert(orig_to_new[&start], new_to_orig[&orig_to_new[&((start + i as usize) % orig_list.len())]]);
         new_to_orig.insert(orig_to_new[&((start + i as usize) % orig_list.len())], tmp);
         orig_to_new.insert(start, mod_add(orig_to_new[&start].clone(), 1, orig_list));
         orig_to_new.insert((start + i as usize) % orig_list.len(), mod_add(orig_to_new[&((start + i as usize) % orig_list.len())].clone(), orig_list.len() - 1, orig_list));
    }
}

fn get_num(num: usize, new_to_orig: &HashMap<usize, usize>, orig_list: &Vec<i32>) -> i32 {
    orig_list[new_to_orig[&((num - 1) % orig_list.len())]].clone()
}

pub fn part_one(input: &str) -> Option<i32> {
    let orig_list: Vec<_> = input.lines().map(|line|
        line.trim().parse::<i32>().unwrap()
    ).collect();
    
    let mut orig_to_new = HashMap::new();
    for i in 0..orig_list.len() {
        orig_to_new.insert(i, i);
    }
    let mut new_to_orig = HashMap::new();
    for i in 0..orig_list.len() {
        new_to_orig.insert(i, i);
    }

    for i in 0..orig_list.len() {
        mix(i, &orig_list, &mut orig_to_new, &mut new_to_orig);
    }

    print!("[");
    for i in 0..orig_list.len() {
        print!("{} ", orig_list[new_to_orig[&i]]);
    }
    print!("]");

    Some(get_num(1000, &new_to_orig, &orig_list) + get_num(2000, &new_to_orig, &orig_list) + get_num(3000, &new_to_orig, &orig_list))

}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), None);
    }
}
