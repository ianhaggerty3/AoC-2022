use std::collections::HashSet;
use std::cmp;

// pub fn

pub fn collect_visible<F>(row: &Vec<u32>, mut add: F) where F: FnMut(usize) {
    let mut current_max: i32 = -1;
    for i in 0..row.len() {
        if (row[i] as i32) > current_max {
            current_max = row[i] as i32;
            add(i);
        }
    }

    current_max = -1;
    for i in (0..row.len()).rev() {
        if (row[i] as i32) > current_max {
            current_max = row[i] as i32;
            add(i);
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut forest: Vec<Vec<u32>> = Vec::new();
    let mut visible: HashSet<(usize, usize)> = HashSet::new();
    for line in input.lines() {
        let heights: Vec<_> = line.chars().map(|char|char.to_digit(10).unwrap()).collect();
        forest.push(heights);
    }

    for (i, row) in forest.iter().enumerate() {
        collect_visible(row, |j| {
            visible.insert((i, j)); });
    }

    for j in 0..forest[0].len() {
        let col: Vec<_> = forest.iter().map(|row| { row[j] }).collect();
        collect_visible(&col, |i| {
            visible.insert((i, j)); })
    }

    println!("{}x{}", forest.len(), forest[0].len());
    println!("visible len: {}", visible.len());
    // println!("{:#?}", visible);


    Some(visible.len() as u32)
}

pub fn get_direction_score(height: u32, heights: &Vec<u32>, index: Vec<usize>) -> u32 {
    let mut score = 0;
    for i in index {
        score += 1;
        if heights[i] >= height {
            break;
        }
    }
    cmp::max(score, 1)
}

pub fn get_score(location: (usize, usize), forest: &Vec<Vec<u32>>) -> u32 {
    let row = &forest[location.0];
    let col: Vec<_> = forest.iter().map(|row| { row[location.1] }).collect();
    let height = forest[location.0][location.1];

    println!("finding score for ({}, {}) = {}", location.0, location.1, height);

    let left_side: Vec<_> = (0..location.1).rev().collect();
    let right_side: Vec<_> = ((location.1 + 1)..forest[0].len()).collect();
    let top_side: Vec<_> = (0..location.0).rev().collect();
    let bottom_side: Vec<_> = ((location.0 + 1)..forest.len()).collect();

    println!("left len = {} right len = {} up len = {} down len = {}", left_side.len(), right_side.len(), top_side.len(), bottom_side.len());


    let score = get_direction_score(height, row, left_side ) *
        get_direction_score(height, row, right_side) *
        get_direction_score(height, &col, top_side) *
        get_direction_score(height, &col, bottom_side);
    println!("score = {}", score);
    score
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut forest: Vec<Vec<u32>> = Vec::new();
    let mut visible: HashSet<(usize, usize)> = HashSet::new();
    for line in input.lines() {
        let heights: Vec<_> = line.chars().map(|char|char.to_digit(10).unwrap()).collect();
        forest.push(heights);
    }

    let mut max_score = 1;
    for (i, row) in forest.iter().enumerate() {
        for (j, tree) in row.iter().enumerate() {
            // 4605120 is too high
            // 55440 is too low
            // 259308 is correct but for a different input, but too low
            max_score = cmp::max(max_score, get_score((i, j), &forest));
        }
    }

    Some(max_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), None);
    }
}
