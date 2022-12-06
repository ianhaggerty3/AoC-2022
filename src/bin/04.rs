pub fn part_one(input: &str) -> Option<u32> {
    let mut full_overlap_total = 0;
    for line in input.lines() {
        let pairs: Vec<_> = line.split(',').collect();

        let a: Vec<_> = pairs[0].split('-').collect();
        let b: Vec<_> = pairs[1].split('-').collect();
        let x1 = a[0].parse::<u32>().unwrap();
        let y1 = a[1].parse::<u32>().unwrap();
        let x2 = b[0].parse::<u32>().unwrap();
        let y2 = b[1].parse::<u32>().unwrap();

        let condition = x1 >= x2 && y1 <= y2 || x1 <= x2 && y1 >= y2;
        full_overlap_total += if x1 >= x2 && y1 <= y2 || x1 <= x2 && y1 >= y2 {
            1
        } else {
            0
        };

        println!("{}, {}: {}", pairs[0], pairs[1], condition)
    }

    Some(full_overlap_total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut overlap_total = 0;
    for line in input.lines() {
        let pairs: Vec<_> = line.split(',').collect();

        let a: Vec<_> = pairs[0].split('-').collect();
        let b: Vec<_> = pairs[1].split('-').collect();
        let x1 = a[0].parse::<u32>().unwrap();
        let y1 = a[1].parse::<u32>().unwrap();
        let x2 = b[0].parse::<u32>().unwrap();
        let y2 = b[1].parse::<u32>().unwrap();

        let condition = x1 <= y2 && y1 >= x2;
        overlap_total += if x1 <= y2 && y1 >= x2 {
            1
        } else {
            0
        };

        println!("{}, {}: {}", pairs[0], pairs[1], condition)
    }

    Some(overlap_total)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), None);
    }
}
