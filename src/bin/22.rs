use std::collections::HashSet;
use std::ops::Range;

fn get_range_from_row(row: &str) -> Range<usize> {
    let mut min = row.len();
    let mut max = 0;
    for (i, token) in row.chars().enumerate() {
        match token {
            '.' | '#' => {
                min = std::cmp::min(min, i);
                max = std::cmp::max(max, i);
            },
            _ => continue,
        }
    }
    min..max
}

fn update_row(row: &str, i: usize, rows: &mut Vec<Range<usize>>) {
   let range = get_range_from_row(row);
   rows[i] = range;
}

fn update_col(i: usize, col: &mut Range<usize>) {
    col.start = std::cmp::min(col.start, i);
    col.end = std::cmp::max(col.end, i);
}

fn update_cols(row: &str, i: usize, cols: &mut Vec<Range<usize>>) {
    for (j, token) in row.chars().enumerate() {
        match token {
            '.' | '#' => {
                update_col(i, &mut cols[j]);
            },
            _ => continue,
        }
    }
}

fn update_blockers(row: &str, i: usize, blockers: &mut HashSet<(usize, usize)>) {
    for (j, token) in row.chars().enumerate() {
        match token {
            '#' => {
                blockers.insert((j, i));
            },
            _ => continue,
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Magnitude(u32),
    Left(),
    Right(),
}

#[derive(Debug)]
enum Direction {
    Up(),
    Down(),
    Left(),
    Right(),
}

fn perform_instruction(instruction: &Instruction, current_pos: &mut (usize, usize), current_direction: &mut Direction, rows: &Vec<Range<usize>>, cols: &Vec<Range<usize>>, blockers: &HashSet<(usize, usize)>) {

    match instruction {
        Instruction::Left() => {
            match current_direction {
                Direction::Up() => {
                    *current_direction = Direction::Left();
                },
                Direction::Down() => {
                    *current_direction = Direction::Right();
                },
                Direction::Left() => {
                    *current_direction = Direction::Down();
                },
                Direction::Right() => {
                    *current_direction = Direction::Up();
                },
            }
        },
        Instruction::Right() => {
            match current_direction {
                Direction::Up() => {
                    *current_direction = Direction::Right();
                },
                Direction::Down() => {
                    *current_direction = Direction::Left();
                },
                Direction::Left() => {
                    *current_direction = Direction::Up();
                },
                Direction::Right() => {
                    *current_direction = Direction::Down();
                },
            }
        },
        Instruction::Magnitude(dist) => {
            let transform_tuple = match current_direction {
                Direction::Up() => (0, -1),
                Direction::Down() => (0, 1),
                Direction::Left() => (-1, 0),
                Direction::Right() => (1, 0),
            };
            for i in 0..*dist {
                let mut proposed_new_x = current_pos.0 as i32 + transform_tuple.0; 
                if proposed_new_x > rows[current_pos.1].end as i32 {
                    proposed_new_x = rows[current_pos.1].start as i32;
                } else if proposed_new_x < rows[current_pos.1].start as i32 {
                    proposed_new_x = rows[current_pos.1].end as i32;
                }

                let mut proposed_new_y = current_pos.1 as i32 + transform_tuple.1; 
                if proposed_new_y > cols[current_pos.0].end as i32 {
                    proposed_new_y = cols[current_pos.0].start as i32;
                } else if proposed_new_y < cols[current_pos.0].start as i32 {
                    proposed_new_y = cols[current_pos.0].end as i32;
                }

                let proposed_new_location = (proposed_new_x as usize, proposed_new_y as usize);
                if blockers.contains(&proposed_new_location) {
                    break;
                }

                println!("moving from {:?} to {:?} facing {:?}", current_pos, proposed_new_location, current_direction);
                *current_pos = proposed_new_location;
            }
        },
    }
}

fn parse_instructions(instructions: &str) -> Vec<Instruction> {
    let mut ret = Vec::new();
    let mut i = 0;
    let tokens: Vec<_> = instructions.trim().chars().collect();
    let mut current_num = String::from("");
    while i < tokens.len() {
        let token = tokens[i];
        match token {
            'L' => {
                ret.push(Instruction::Magnitude(current_num.parse::<u32>().unwrap()));
                current_num = String::from("");
                ret.push(Instruction::Left());
            },
            'R' => {
                ret.push(Instruction::Magnitude(current_num.parse::<u32>().unwrap()));
                current_num = String::from("");
                ret.push(Instruction::Right());
            }
            _ => current_num.push(token),
        }
        i += 1;
    }
    ret.push(Instruction::Magnitude(current_num.parse::<u32>().unwrap()));

    ret
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut rows: Vec<Range<usize>> = Vec::new();
    let mut cols: Vec<Range<usize>> = Vec::new();
    let mut blockers: HashSet<(usize, usize)> = HashSet::new();
    let input_vec: Vec<_> = input.lines().collect();

    for (i, row) in input.lines().enumerate() {
        if row.len() == 0 {
            break;
        }
        rows.push(0..0);
        update_row(row, i, &mut rows);
    }

    let max_col = rows.iter().map(|range|range.end).max().unwrap();

    for i in 0..=max_col {
        cols.push(rows.len()..0); // impossible range as initial value
    }

    for (i, row) in input.lines().enumerate() {
        if row.len() == 0 {
            break;
        }
        update_cols(row, i, &mut cols);
    }

    let mut final_index = 0;
    for (i, row) in input.lines().enumerate() {
        if row.len() == 0 {
            final_index = i + 1;
            break;
        }
        update_blockers(row, i, &mut blockers);
    }

    let instructions = parse_instructions(input_vec[final_index]);
    
    //println!("{:?}", cols);
    //println!("{:?}", rows);
    println!("{:?}", blockers);
    //println!("{:?}", instructions);

    let mut current_pos = (rows[0].start, cols[rows[0].start].start);
    let mut current_direction = Direction::Right();
    for instruction in &instructions {
        perform_instruction(instruction, &mut current_pos, &mut current_direction, &rows, &cols, &blockers);
    }

    let dir_score = match current_direction {
        Direction::Up() => 3,
        Direction::Down() => 1,
        Direction::Left() => 2,
        Direction::Right() => 0,
    };
    let score = (current_pos.1 as u32 + 1) * 1000 + (current_pos.0 as u32 + 1) * 4 + dir_score;

    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 22);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_one(&input), Some(6032));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_two(&input), None);
    }
}
