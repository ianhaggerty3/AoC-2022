use std::collections::HashMap;

pub fn process_command<'a>(command: &'a str, current_dir: &mut Vec<&'a str>) -> () {
    if command == "$ ls" {
        current_dir;
    } else if command == "$ cd /" {
        current_dir.clear();
        println!("clearing!");
        current_dir;
    } else if command == "$ cd .." {
        println!("popping");
        current_dir.pop();
        current_dir;
    } else if command.starts_with("$ cd") {
        let parts: Vec<_> = command.split(" ").collect();
        current_dir.push(parts[2]);
        current_dir;
    } else {
        panic!("Unexpected command");
    }
}

pub fn process_file(line: &str, file_map: &mut HashMap<String, Vec<(String, u32)>>, current_dir: &Vec<&str>) -> () {
    let parts: Vec<_> = line.split(" ").collect();
    let size = parts[0].parse::<u32>().unwrap();
    let file = parts[1];

    let dir_name = current_dir.join("/");
    println!("Adding {}/{} for {}", dir_name, file, size);
    let entry = file_map.get_mut(&dir_name);
    if let Some(mut entry) = entry {
        if entry.iter().find(|&pair| pair.0 == file).is_none() {
            entry.push((file.to_owned(), size));
        }
    } else {
        file_map.insert(dir_name, Vec::from([(file.to_owned(), size)]));
    }
}

pub fn process_dir(line: &str, file_map: &mut HashMap<String, Vec<(String, u32)>>, current_dir: &Vec<&str>) -> () {
    let parts: Vec<_> = line.split(" ").collect();
    let new_dir = parts[1];

    let dir_name = current_dir.join("/");
    println!("Adding dir {}/{}", dir_name, new_dir);
    let entry = file_map.get_mut(&dir_name);
    if let Some(mut entry) = entry {
        if entry.iter().find(|&pair| pair.0 == new_dir).is_none() {
            entry.push((new_dir.to_owned(), 0));
        }
    } else {
        file_map.insert(dir_name, Vec::from([(new_dir.to_owned(), 0)]));
    }
}

pub fn is_command(line: &str) -> bool {
    line.starts_with("$")
}

pub fn is_file(line: &str) -> bool {
    !line.starts_with("dir")
}

pub fn get_size(dir: &str, file_map: &HashMap<String, Vec<(String, u32)>>) -> u32 {
    let entry = file_map.get(dir);
    if entry.is_none() {
        println!("get_size returning early");
        return 0;
    }
    let entry = entry.unwrap();
    let mut total_size = 0;
    for entity in entry {
        if entity.1 == 0 {
            println!("get_size recursing into {}", dir.to_owned() + "/" + &entity.0);
            total_size += get_size(&(dir.to_owned() + "/" + &entity.0), file_map);
        } else {
            total_size += entity.1;
        }
    }

    total_size
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut file_map: HashMap<String, Vec<(String, u32)>> = HashMap::new();
    let mut current_dir = vec!["/"];

    for line in input.lines() {
        let line = line.trim();
        if is_command(line) {
            process_command(line, &mut current_dir);
        } else {
            if is_file(line) {
                process_file(line, &mut file_map, &current_dir);
            } else {
                process_dir(line, &mut file_map, &current_dir);
            }
        }
    }
    // println!("{:#?}", &file_map);

    //          40483429
    // too high 34029713
    // too low  1642445
    // too high 2783210
    // 1915606
    println!();
    let mut total_size = 0;
    for (name, pairs) in &file_map {
        let dir_size = get_size(&name, &file_map);
        if dir_size <= 100000 {
            total_size += dir_size;
        }
    }

    Some(total_size)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut file_map: HashMap<String, Vec<(String, u32)>> = HashMap::new();
    let mut current_dir = vec!["/"];

    for line in input.lines() {
        let line = line.trim();
        if is_command(line) {
            process_command(line, &mut current_dir);
        } else {
            if is_file(line) {
                process_file(line, &mut file_map, &current_dir);
            } else {
                process_dir(line, &mut file_map, &current_dir);
            }
        }
    }

    let mut total_size = 0;
    for (name, pairs) in &file_map {
        for pair in pairs {
            total_size += pair.1;
        }
    }

    let total_filesystem_size = 70000000;
    let desired_free_space = 30000000;
    let current_free_space = total_filesystem_size - total_size;
    let space_to_free = desired_free_space - current_free_space;

    let mut eligible_dirs: Vec<(String, u32)> = Vec::new();
    for (name, pairs) in &file_map {
        let dir_size = get_size(&name, &file_map);
        if dir_size >= space_to_free {
            eligible_dirs.push((name.to_owned(), dir_size));
        }
    }

    eligible_dirs.sort_by(|a, b|a.1.partial_cmp(&b.1).unwrap());
    // println!("{:#?}", eligible_dirs);
    println!("{}", eligible_dirs[0].0);
    println!("{}, {}", current_free_space, space_to_free);

    Some(eligible_dirs[0].1)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
