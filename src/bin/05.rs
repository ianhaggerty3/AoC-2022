use sscanf::sscanf;

pub fn get_num_rows(input: &str) -> usize {
    let mut num_rows = 0;
    for line in input.lines() {
        if !line.contains('[') {
            break;
        }
        num_rows += 1;
    }

    num_rows
}

pub fn get_num_cols(input: &str) -> usize {
    let mut index_line = "";
    for line in input.lines() {
        if !line.contains('[') {
            index_line = line;
            break;
        }
    }

    let mut index_nums: Vec<_> = index_line.clone().split(' ').collect();
    index_nums.reverse();
    index_nums[0].parse::<usize>().unwrap()
}

pub fn get_col_index(col: usize) -> usize {
    1 + 4 * col
}

pub fn part_one(input: &str) -> Option<u32> {
    let num_rows = get_num_rows(input);
    let num_cols = get_num_cols(input);
    let lines: Vec<_> = input.lines().collect();

    println!("{}, {}", num_rows, num_cols);

    let mut columns: Vec<Vec<char>> = Vec::new();
    for i in 0..num_cols {
        columns.push(Vec::new());
        for j in (0..num_rows).rev() {
            let current_char = lines[j].chars().nth(get_col_index(i));
            if let Some(current_char) = current_char {
                if current_char != ' ' {
                    columns[i].push(current_char);
                    println!("Adding: {} to {}, {}", current_char, j, i);
                }
            }
        }
    }

    let lines: Vec<_> = input.lines().collect();
    for line in &lines[(num_cols + 1)..] {
        println!("{}", line);
        let parsed = sscanf::sscanf!(line, "move {u32} from {usize} to {usize}").unwrap();
        let num_moved = parsed.0;
        let col1 = parsed.1 - 1;
        let col2 = parsed.2 - 1;

        for i in 0..num_moved {
            // println!("{}", i);
            let mut col: &mut Vec<_> = &mut columns[col1];
            let val = col.pop().unwrap();
            let mut col: &mut Vec<_> = &mut columns[col2];
            col.push(val);
        }

        for i in 0..num_cols {
            print!("{} ", columns[i].len());
        }
        println!();
    }

    println!("Answer");
    for i in 0..num_cols {
        columns[i].reverse();
        print!("{}", columns[i][0])
    }
    println!();
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let num_rows = get_num_rows(input);
    let num_cols = get_num_cols(input);
    let lines: Vec<_> = input.lines().collect();

    println!("{}, {}", num_rows, num_cols);

    let mut columns: Vec<Vec<char>> = Vec::new();
    for i in 0..num_cols {
        columns.push(Vec::new());
        for j in (0..num_rows).rev() {
            let current_char = lines[j].chars().nth(get_col_index(i));
            if let Some(current_char) = current_char {
                if current_char != ' ' {
                    columns[i].push(current_char);
                    println!("Adding: {} to {}, {}", current_char, j, i);
                }
            }
        }
    }

    let lines: Vec<_> = input.lines().collect();
    for line in &lines[(num_cols + 1)..] {
        println!("{}", line);
        let parsed = sscanf::sscanf!(line, "move {u32} from {usize} to {usize}").unwrap();
        let num_moved = parsed.0;
        let col1 = parsed.1 - 1;
        let col2 = parsed.2 - 1;

        let base = columns[col1].len() - num_moved as usize;
        for i in 0..num_moved {
            // println!("{}", i);
            let mut col: &mut Vec<_> = &mut columns[col1];
            let val = col[base];
            col.remove(base);
            let mut col: &mut Vec<_> = &mut columns[col2];
            col.push(val);
        }

        for i in 0..num_cols {
            print!("{} ", columns[i].len());
        }
        println!();
    }

    println!("Answer 2");
    for i in 0..num_cols {
        columns[i].reverse();
        print!("{}", columns[i][0])
    }
    println!();
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }
}
