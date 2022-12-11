fn is_interesting(cycle: i32) -> bool {
    (cycle - 20) % 40 == 0 && cycle <= 220
}

fn process_instruction(instruction: &str, X: &mut i32, cycle: &mut i32, interesting_strengths: &mut Vec<i32>) {
    if instruction == "noop" {
        if is_interesting(cycle.clone()) {
            interesting_strengths.push(cycle.clone() as i32 * X.clone());
        }
        *cycle += 1;
    } else {
        let parts: Vec<_> = instruction.split(" ").collect();
        let magnitude = parts[1].parse::<i32>().unwrap();
        for i in 0..2 {
            if is_interesting(cycle.clone()) {
                interesting_strengths.push(cycle.clone() * X.clone());
            }
            *cycle += 1;
        }
        *X += magnitude;
        println!("{}", X);
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut cycle = 1;
    let mut X = 1;
    let mut interesting_strengths: Vec<i32> = Vec::new();
    for line in input.lines() {
        process_instruction(line, &mut cycle, &mut X, &mut interesting_strengths);
    }


    Some(interesting_strengths.iter().sum())
}

fn is_visibile(cycle: i32, X: i32) -> bool {
    ((cycle % 40) - 1 - X).abs() <= 1
}

fn sim_crt(instruction: &str, X: &mut i32, cycle: &mut i32, visible_history: &mut Vec<bool>) {
    if instruction == "noop" {
        visible_history.push(is_visibile(cycle.clone(),X.clone()));
        *cycle += 1;
    } else {
        let parts: Vec<_> = instruction.split(" ").collect();
        let magnitude = parts[1].parse::<i32>().unwrap();
        for i in 0..2 {
            visible_history.push(is_visibile(cycle.clone(),X.clone()));
            *cycle += 1;
        }
        *X += magnitude;
        println!("{}", X);
    }
}

fn print_screen(visible_history: Vec<bool>) {
    for i in 0..6 {
        for j in 0..40 {
            let mut char = ".";
            if visible_history[(i*40)+j] {
                char = "#";
            }
            print!("{}", char)
        }
        println!();
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cycle = 1;
    let mut X = 1;
    let mut visible_history: Vec<bool> = Vec::new();
    for line in input.lines() {
        sim_crt(line, &mut cycle, &mut X, &mut visible_history);
    }

    println!("len = {}", visible_history.len());
    print_screen(visible_history);

    // not PLGFKAZB

    Some(0)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
