use std::collections::HashMap;

fn mod_add(a: usize, b: usize, modulus: usize) -> usize {
    (a + b) % modulus
}

fn mix(start: usize, orig_list: &Vec<i64>, orig_to_encrypted: &mut HashMap<usize, usize>, encrypted_to_orig: &mut HashMap<usize, usize>) {
    let length = orig_list.len();
    let cur = orig_list[start];
    let cur = cur.rem_euclid(length as i64 - 1); // it takes length - 1 swaps to get to the same place as before
   //  println!("start = {}, encrypted_start = {}, cur = {}", start, orig_to_encrypted[&start], cur);

    for _i in 1..=cur {
         let tmp = encrypted_to_orig[&orig_to_encrypted[&start]];
         encrypted_to_orig.insert(orig_to_encrypted[&start], encrypted_to_orig[&mod_add(orig_to_encrypted[&start], 1, length)]);
         encrypted_to_orig.insert(mod_add(orig_to_encrypted[&start], 1, length), tmp);

         let other_orig_index = encrypted_to_orig[&orig_to_encrypted[&start]];
         orig_to_encrypted.insert(start, mod_add(orig_to_encrypted[&start].clone(), 1, length));
         orig_to_encrypted.insert(other_orig_index, mod_add(orig_to_encrypted[&other_orig_index].clone(), orig_list.len() - 1, length));
    }
}

fn get_num(num: usize, encrypted_to_orig: &HashMap<usize, usize>, orig_list: &Vec<i64>) -> i64 {
    orig_list[encrypted_to_orig[&(num % orig_list.len())]].clone()
}

fn print_new_list(orig_list: &Vec<i64>, encrypted_to_orig: &HashMap<usize, usize>) {
    print!("[");
    for i in 0..orig_list.len() {
        print!("{}, ", orig_list[encrypted_to_orig[&i]]);
    }
    print!("]");
    println!();
}

fn verify_structs(orig_list: &Vec<i64>, orig_to_encrypted: &HashMap<usize, usize>, encrypted_to_orig: &HashMap<usize, usize>) {
    for i in 0..orig_list.len() {
        assert_eq!(i, orig_to_encrypted[&encrypted_to_orig[&i]].clone());
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let orig_list: Vec<_> = input.lines().map(|line|
        line.trim().parse::<i64>().unwrap()
    ).collect();
    
    let mut orig_to_encrypted = HashMap::new();
    for i in 0..orig_list.len() {
        orig_to_encrypted.insert(i, i);
    }
    let mut encrypted_to_orig = HashMap::new();
    for i in 0..orig_list.len() {
        encrypted_to_orig.insert(i, i);
    }

    // print_new_list(&orig_list, &encrypted_to_orig);
    
    for i in 0..orig_list.len() {
        mix(i, &orig_list, &mut orig_to_encrypted, &mut encrypted_to_orig);
        verify_structs(&orig_list, &orig_to_encrypted, &encrypted_to_orig);
      //   print_new_list(&orig_list, &encrypted_to_orig);
    }

    let zero_orig_index = orig_list.iter().position(|&x| x == 0).unwrap().clone();
    let zero_encrypted_index = orig_to_encrypted[&zero_orig_index];
    // println!("{:?}", orig_list);
    println!("zero orig = {} encrypted = {}", zero_orig_index, zero_encrypted_index);
    let num1 = 1000 + zero_encrypted_index;
    let num2 = 2000 + zero_encrypted_index;
    let num3 = 3000 + zero_encrypted_index;

    println!("{} + {} + {}", get_num(num1, &encrypted_to_orig, &orig_list), get_num(num2, &encrypted_to_orig, &orig_list), get_num(num3, &encrypted_to_orig, &orig_list));
    Some(get_num(num1, &encrypted_to_orig, &orig_list) + get_num(num2, &encrypted_to_orig, &orig_list) + get_num(num3, &encrypted_to_orig, &orig_list))
}

pub fn part_two(input: &str) -> Option<i64> {
    let orig_list: Vec<_> = input.lines().map(|line|
        line.trim().parse::<i64>().unwrap()
    ).collect();
    let orig_list: Vec<_> = orig_list.iter().map(|item|item * 811589153).collect();
    
    let mut orig_to_encrypted = HashMap::new();
    for i in 0..orig_list.len() {
        orig_to_encrypted.insert(i, i);
    }
    let mut encrypted_to_orig = HashMap::new();
    for i in 0..orig_list.len() {
        encrypted_to_orig.insert(i, i);
    }

    // print_new_list(&orig_list, &encrypted_to_orig);
    
    for i in 0..orig_list.len() * 10 {
        let i = i % orig_list.len();
        mix(i, &orig_list, &mut orig_to_encrypted, &mut encrypted_to_orig);
        verify_structs(&orig_list, &orig_to_encrypted, &encrypted_to_orig);
      //   print_new_list(&orig_list, &encrypted_to_orig);
    }

    let zero_orig_index = orig_list.iter().position(|&x| x == 0).unwrap().clone();
    let zero_encrypted_index = orig_to_encrypted[&zero_orig_index];
    // println!("{:?}", orig_list);
    println!("zero orig = {} encrypted = {}", zero_orig_index, zero_encrypted_index);
    let num1 = 1000 + zero_encrypted_index;
    let num2 = 2000 + zero_encrypted_index;
    let num3 = 3000 + zero_encrypted_index;

    println!("{} + {} + {}", get_num(num1, &encrypted_to_orig, &orig_list), get_num(num2, &encrypted_to_orig, &orig_list), get_num(num3, &encrypted_to_orig, &orig_list));
    Some(get_num(num1, &encrypted_to_orig, &orig_list) + get_num(num2, &encrypted_to_orig, &orig_list) + get_num(num3, &encrypted_to_orig, &orig_list))
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
        assert_eq!(part_two(&input), Some(1623178306));
    }
}
