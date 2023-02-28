use std::collections::HashSet;
use std::collections::HashMap;
use sscanf::sscanf;

#[derive(Debug)]
struct Blueprint {
    ore_bot_cost: u32,
    clay_bot_cost: u32,
    obsidian_bot_cost: (u32, u32),
    geode_bot_cost: (u32, u32),
}

fn get_blueprint(line: &str) -> Blueprint {
    let parsed = sscanf::sscanf!(line, "Blueprint {u32}: Each ore robot costs {u32} ore. Each clay robot costs {u32} ore. Each obsidian robot costs {u32} ore and {u32} clay. Each geode robot costs {u32} ore and {u32} obsidian.").unwrap();
    Blueprint {
        ore_bot_cost: parsed.1,
        clay_bot_cost: parsed.2,
        obsidian_bot_cost: (parsed.3, parsed.4),
        geode_bot_cost: (parsed.5, parsed.6)
    }
}

fn get_neighbors(current: (u32, u32, u32, u32, u32, u32, u32, u32, u32, bool, bool, bool), blueprint: &Blueprint, days: u32) -> Vec<(u32, u32, u32, u32, u32, u32, u32, u32, u32, bool, bool, bool)> {
    let mut ret: Vec<(u32, u32, u32, u32, u32, u32, u32, u32, u32, bool, bool, bool)> = Vec::new();
    let mut ore_bot_possible = false;
    let mut clay_bot_possible = false;
    let mut obsidian_bot_possible = false;

    if current.0 == days {
        return ret;
    }

    // buy geode
    if current.2 >= blueprint.geode_bot_cost.0 && current.6 >= blueprint.geode_bot_cost.1 {
        ret.push((current.0 + 1, current.1, current.2 - blueprint.geode_bot_cost.0 + current.1, current.3, current.4 + current.3, current.5, current.6 - blueprint.geode_bot_cost.1 + current.5, current.7 + 1, current.8 + current.7, false, false, false));
        return ret;
    }

    let max_ore_cost = blueprint.clay_bot_cost.max(blueprint.obsidian_bot_cost.0).max(blueprint.geode_bot_cost.0);

    // buy ore
    if !current.9 && current.2 >= blueprint.ore_bot_cost && current.1 < max_ore_cost {
        ore_bot_possible = false;
        ret.push((current.0 + 1, current.1 + 1, current.2 - blueprint.ore_bot_cost + current.1, current.3, current.4 + current.3, current.5, current.6 + current.5, current.7, current.8 + current.7, false, false, false));
    }

    // buy clay
    if !current.10 && current.2 >= blueprint.clay_bot_cost {
        clay_bot_possible = true;
        ret.push((current.0 + 1, current.1, current.2 - blueprint.clay_bot_cost + current.1, current.3 + 1, current.4 + current.3, current.5, current.6 + current.5, current.7, current.8 + current.7, false, false, false));
    }

    // buy obsidian
    if !current.11 && current.2 >= blueprint.obsidian_bot_cost.0 && current.4 >= blueprint.obsidian_bot_cost.1 {
        obsidian_bot_possible = true;
        ret.push((current.0 + 1, current.1, current.2 - blueprint.obsidian_bot_cost.0 + current.1, current.3, current.4 - blueprint.obsidian_bot_cost.1 + current.3, current.5 + 1, current.6 + current.5, current.7, current.8 + current.7, false, false, false));
    }

    // if (current.5 == 0 && !obsidian_bot_possible || current.5 != 0) && (current.3 == 0 && !clay_bot_possible || current.3 != 0) {
    ret.push((current.0 + 1, current.1, current.2 + current.1, current.3, current.4 + current.3, current.5, current.6 + current.5, current.7, current.8 + current.7, current.9 || ore_bot_possible, current.10 || clay_bot_possible, current.11 || obsidian_bot_possible));
    // }

    ret
}

fn find_max_geodes(blueprint: &Blueprint, days: u32) -> u32 {
    let mut open_set: HashSet<(u32, u32, u32, u32, u32, u32, u32, u32, u32, bool, bool, bool)> = HashSet::new();
    let mut best_path_geodes = 0;
    let mut current_best_geodes = [0; 32];
    open_set.insert((0, 1, 0, 0, 0, 0, 0, 0, 0, false, false, false));

    while !open_set.is_empty() {
        let current = open_set.iter().next().unwrap().clone();
        open_set.remove(&current);

        for neighbor in get_neighbors(current.clone(), blueprint, days) {
            let new_actual_geodes = neighbor.8;
            if new_actual_geodes < current_best_geodes[(neighbor.0 - 1) as usize] {
                continue;
            }

            best_path_geodes = std::cmp::max(best_path_geodes ,new_actual_geodes);

            current_best_geodes[(neighbor.0 - 1) as usize] = new_actual_geodes;
            open_set.insert(neighbor.clone());
        }
    }

    println!("best_path_geodes = {}", best_path_geodes);

    best_path_geodes
}


pub fn part_one(input: &str) -> Option<u32> {
    let mut blueprints: Vec<Blueprint> = Vec::new();
    for line in input.lines() {
        blueprints.push(get_blueprint(line));
    }

    let mut quality_score = 0;
    for (i, blueprint) in blueprints.iter().enumerate() {
        println!("starting search #{}", i);
        quality_score = quality_score + (i as u32 + 1) * find_max_geodes(blueprint, 24);
    }

    Some(quality_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut blueprints: Vec<Blueprint> = Vec::new();
    for line in input.lines() {
        blueprints.push(get_blueprint(line));
    }

    let mut geode_product = 1;
    for (i, blueprint) in blueprints.iter().take(3).enumerate() {
        println!("starting search #{}", i);
        geode_product *= find_max_geodes(blueprint, 32);
    }

    Some(geode_product)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_one(&input), Some(33));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_two(&input), Some(56 * 62));
    }
}
