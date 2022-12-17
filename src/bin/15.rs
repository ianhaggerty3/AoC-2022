use std::panic::PanicInfo;
use sscanf::sscanf;

#[derive(Debug)]
struct Circle {
    center: (i32, i32),
    radius: u32
}

#[derive(Debug)]
struct Bounds {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}

fn update_bounds(x: i32, y: i32, bounds: &mut Bounds) {
    bounds.x1 = bounds.x1.min(x);
    bounds.x2 = bounds.x2.max(x);
    bounds.y1 = bounds.y1.min(y);
    bounds.y2 = bounds.y2.max(y);
}

fn intersects(point: (i32, i32), circle: &Circle) -> bool {
    let x_diff = (point.0 - circle.center.0).abs() as u32;
    let y_diff = (point.1 - circle.center.1).abs() as u32;

    if x_diff + y_diff <= circle.radius {
    } else {
    }

    x_diff + y_diff <= circle.radius
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut circles: Vec<Circle> = Vec::new();
    let mut beacons: Vec<(i32, i32)> = Vec::new();
    let mut bounds: Bounds = Bounds { x1: 0, x2: 0, y1: 0, y2: 0 };
    for line in input.lines() {
        let parsed = sscanf::sscanf!(line, "Sensor at x={i32}, y={i32}: closest beacon is at x={i32}, y={i32}").unwrap();
        let x1: i32 = parsed.0;
        let y1: i32 = parsed.1;
        let x2: i32 = parsed.2;
        let y2: i32 = parsed.3;

        let radius = ((x1 - x2).abs() + (y1 - y2).abs()) as u32;
        update_bounds(x1 - (radius as i32), y1 - (radius as i32), &mut bounds);
        update_bounds(x1 + (radius as i32), y1 + (radius as i32), &mut bounds);
        update_bounds(x2 - (radius as i32), y2 - (radius as i32), &mut bounds);
        update_bounds(x2 + (radius as i32), y2 + (radius as i32), &mut bounds);

        circles.push(Circle { center: (x1, y1), radius });
        beacons.push((x2, y2));
    }

    println!("{:?}", bounds);

    let mut not_possible_count = 0;
    for x in bounds.x1..(bounds.x2 + 1) {
        let y = 2000000;
        if circles.iter().any(|circle| intersects((x, y), circle)) {
            if !beacons.contains(&(x, y)) {
                not_possible_count += 1;
            }
        }
    }

    // 4512265 too low
    // 5525848 too high

    Some(not_possible_count)
}

fn get_next_free(x: i32, y: i32, circle: &Circle) -> i32 {
    let y_diff = (y - circle.center.1).abs();
    let x_diff = (x - circle.center.0).abs();
    x + ((circle.radius as i32) - y_diff - x_diff) + 1
}

pub fn part_two(input: &str) -> Option<i64> {

    let mut circles: Vec<Circle> = Vec::new();
    let mut beacons: Vec<(i32, i32)> = Vec::new();
    for line in input.lines() {
        let parsed = sscanf::sscanf!(line, "Sensor at x={i32}, y={i32}: closest beacon is at x={i32}, y={i32}").unwrap();
        let x1: i32 = parsed.0;
        let y1: i32 = parsed.1;
        let x2: i32 = parsed.2;
        let y2: i32 = parsed.3;

        let radius = ((x1 - x2).abs() + (y1 - y2).abs()) as u32;

        circles.push(Circle { center: (x1, y1), radius });
        beacons.push((x2, y2));
    }

    let bounds: Bounds = Bounds { x1: 0, x2: 4000000, y1: 0, y2: 4000000 };

    let mut target_x = 0;
    let mut target_y = 0;

    for y in bounds.y1..(bounds.y2 + 1) {
        let mut x = 0;
        while x <= bounds.x2 {
            let mut does_intersect = false;
            for circle in &circles {
                if intersects((x, y), &circle) {
                    x = get_next_free(x, y, &circle);
                    does_intersect = true;
                    break;
                }
            }
            if !does_intersect {
                println!("hidden beacon found at {}, {}", x, y);
                target_x = x;
                target_y = y;
                break;
            }
        }
    }
    Some((target_x as i64) * 4000000 + (target_y as i64))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), None);
    }
}
