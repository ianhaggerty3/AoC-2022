use std::collections::{HashMap, HashSet};
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

fn perform_instruction(instruction: &Instruction, current_pos: &mut (usize, usize), current_direction: &mut i32, rows: &Vec<Range<usize>>, cols: &Vec<Range<usize>>, blockers: &HashSet<(usize, usize)>) {

    match instruction {
        Instruction::Left() => *current_direction = (*current_direction - 1).rem_euclid(4),
        Instruction::Right() => *current_direction = (*current_direction + 1).rem_euclid(4),
        Instruction::Magnitude(dist) => {
            let transform_tuple = match current_direction {
                0 => (1, 0),
                1 => (0, 1),
                2 => (-1, 0),
                3 => (0, -1),
                _ => panic!("unexpected direction"),
            };
            for _i in 0..*dist {
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

                //println!("moving from {:?} to {:?} facing {:?}", current_pos, proposed_new_location, current_direction);
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

fn get_score(current_direction: &i32, current_pos: &(usize, usize)) -> u32 {
    (current_pos.1 as u32 + 1) * 1000 + (current_pos.0 as u32 + 1) * 4 + (*current_direction as u32)
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

    for _i in 0..=max_col {
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
    
    let mut current_pos = (rows[0].start, cols[rows[0].start].start);
    let mut current_direction = 0;
    for instruction in &instructions {
        perform_instruction(instruction, &mut current_pos, &mut current_direction, &rows, &cols, &blockers);
    }

    let score = get_score(&current_direction, &current_pos);

    Some(score)
}

fn perform_cube_instruction(instruction: &Instruction, current_pos: &mut (usize, usize), current_direction: &mut i32, rows: &Vec<Range<usize>>, cols: &Vec<Range<usize>>, blockers: &HashSet<(usize, usize)>) {

    match instruction {
        Instruction::Left() => {
            *current_direction = (*current_direction - 1).rem_euclid(4);
        },
        Instruction::Right() => {
            *current_direction = (*current_direction + 1).rem_euclid(4);
        },
        Instruction::Magnitude(dist) => {
            let transform_tuple = match current_direction {
                0 => (1, 0),
                1 => (0, 1),
                2 => (-1, 0),
                3 => (0, -1),
                _ => panic!("unexpected direction"),
            };
            for _i in 0..*dist {
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

                //println!("moving from {:?} to {:?} facing {:?}", current_pos, proposed_new_location, current_direction);
                *current_pos = proposed_new_location;
            }
        },
    }
}

fn get_grid_index(pos: &(usize, usize), horizontal_blocks: &usize) -> usize {
    pos.0 + (pos.1 * horizontal_blocks)
}

#[derive(Clone, Debug)]
struct Face {
    neighbors: [Option<usize>; 4],
}

fn get_direction_tuples() -> Vec<(i32, i32)> {
    Vec::from([(1, 0), (0, 1), (-1, 0), (0, -1)])
}

fn missing_edges(faces: &HashMap<usize, Face>) -> bool {
    for face in faces.values() {
       for direction in face.neighbors {
           if direction.is_none() {
               return true;
           }
       }
    }

    false
}

fn get_one_missing_edge(faces: &HashMap<usize, Face>) -> (usize, usize, usize, usize) {
    for (face_index, face) in faces {
        for (direction, neighbor) in face.neighbors.iter().enumerate() {
            if !neighbor.is_none() {
                continue;
            }

            let direction_options = [1, 3];
            for option in direction_options {
                let common_face_index = face.neighbors[(direction + option) % 4];
                if common_face_index.is_none() {
                    continue;
                }
                let common_face_index = common_face_index.unwrap();
                let common_face = faces.get(&common_face_index).unwrap();
                let common_face_direction = common_face.neighbors.iter().position(|&x| x == Some(*face_index)).unwrap();

                let unknown_face_index = common_face.neighbors[(common_face_direction + option) % 4];
                if unknown_face_index.is_none() {
                    continue;
                }
                let unknown_face_index = unknown_face_index.unwrap();
                let unknown_face = faces.get(&unknown_face_index).unwrap();
                let unknown_face_direction = unknown_face.neighbors.iter().position(|&x| x == Some(common_face_index)).unwrap();

                // in this case, we know the unknown face is connected to the original face
                return (face_index.clone(), direction, unknown_face_index, (unknown_face_direction + option) % 4);
            }
        }
    }

    println!("final faces = {:?}", faces);
    panic!("no missing edge found");
}

fn build_grid_index_to_cube_face(rows: &Vec<Range<usize>>) -> HashMap<usize, Face> {
    let mut faces = HashMap::new();
    let max_x = rows.iter().map(|range|range.end).max().unwrap() + 1;
    let max_y = rows.len();

    // assumes 3x4 or 4x3 (not sure if valid for all folding patterns)
    let min_side = std::cmp::min(max_x, max_y);
    let block_size = min_side / 3;
    let horizontal_blocks = max_x / block_size;

    let start = (rows[0].start, 0);
    let mut Q = Vec::from([start]);

    // populate initial face associations
    while Q.len() != 0 {
        let current = Q.pop().unwrap();
        for (i, direction) in get_direction_tuples().iter().enumerate() {
            let possible_new = (current.0 as i32 + (block_size as i32) * direction.0, current.1 as i32 + (block_size as i32) * direction.1);
            if possible_new.1 < 0 || possible_new.1 >= rows.len() as i32 || !rows[possible_new.1 as usize].contains(&(possible_new.0 as usize)) {
                continue;
            }

            let current_index = get_grid_index(&(current.0 / block_size, current.1 / block_size), &horizontal_blocks);
            let mut current_face = faces.get(&current_index).unwrap_or(&(Face { neighbors: [None, None, None, None] } )).clone();
            let other_index = get_grid_index(&(possible_new.0 as usize / block_size, possible_new.1 as usize / block_size), &horizontal_blocks);
            let mut other_face = faces.get(&other_index).unwrap_or(&(Face { neighbors: [None, None, None, None] } )).clone();
            current_face.neighbors[i] = Some(other_index);
            other_face.neighbors[(i + 2) % 4] = Some(current_index);

            if !Q.contains(&(possible_new.0 as usize, possible_new.1 as usize)) && !faces.contains_key(&other_index) {
                Q.push((possible_new.0 as usize, possible_new.1 as usize));
            }
            faces.insert(current_index, current_face.clone());
            faces.insert(other_index, other_face.clone());
        }
    }

    println!("pre-corner analysis: {:?} keys.len() = {}", faces, faces.keys().len());
    while missing_edges(&faces) {
        let (orig_index, orig_direction, new_index, new_direction) = get_one_missing_edge(&faces); 
        println!("orig_index = {}, orig_direction = {}, new_index = {}, new_direction = {}", orig_index, orig_direction, new_index, new_direction);
        let mut face = faces.get_mut(&orig_index).unwrap();
        face.neighbors[orig_direction] = Some(new_index);
        let mut face = faces.get_mut(&new_index).unwrap();
        face.neighbors[new_direction] = Some(orig_index);
    }
    println!("post-corner analysis: {:?} keys.len() = {}", faces, faces.keys().len());
    faces
}

pub fn part_two(input: &str) -> Option<u32> {
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

    build_grid_index_to_cube_face(&rows);
    let instructions = parse_instructions(input_vec[final_index]);
    
    let mut current_pos = (rows[0].start, cols[rows[0].start].start);
    let mut current_direction = 0;
    for instruction in &instructions {
        perform_cube_instruction(instruction, &mut current_pos, &mut current_direction, &rows, &cols, &blockers);
    }

    let score = get_score(&current_direction, &current_pos);

    Some(score)
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
        assert_eq!(part_two(&input), Some(5031));
    }
}
