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
                blockers.insert((i, j));
            },
            _ => continue,
        }
    }
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

    for (i, row) in input.lines().enumerate() {
        if row.len() == 0 {
            break;
        }
        update_blockers(row, i, &mut blockers);
    }

    println!("{:?}", cols);
    println!("{:?}", rows);
    println!("{:?}", blockers);

    None
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
