use std::collections::HashSet;
use std::time::Instant;

const INPUT_FILE: &str = "input1.txt";

pub fn solve() {
    let mut start = Instant::now();
    let raw_input: Vec<String> = aoc_utils::input::load_input_lines(INPUT_FILE);

    let mut left_group: Vec<u32> = Vec::new();
    let mut right_group: Vec<u32> = Vec::new();
    for line in raw_input {
        let parts: Vec<u32> = line.split("   ").map(|s| s.parse::<u32>().unwrap()).collect(); // 3-space delimited
        left_group.push(*parts.get(0).unwrap());
        right_group.push(*parts.get(1).unwrap());
    }
    left_group.sort();
    right_group.sort();

    let setup_time = start.elapsed();
    println!("Setup: {:.2?}", setup_time);
    start = Instant::now();
    solve_part_one(&left_group, &right_group);
    let p1_time = start.elapsed();
    println!("Part 1: {:.2?}", setup_time + p1_time);
    start = Instant::now();
    solve_part_two(&left_group, &right_group);
    let p2_time = start.elapsed();
    println!("Part 2: {:.2?}", setup_time + p2_time);
}

fn solve_part_one(left_group: &Vec<u32>, right_group: &Vec<u32>) {
    let mut solution = 0;
    for i in 0..left_group.len() {
        let left = left_group.get(i).unwrap();
        let right = right_group.get(i).unwrap();
        solution += left.abs_diff(*right);
    }
    println!("Day 1 Part 1: {}", solution);
}

fn solve_part_two(left_group: &Vec<u32>, right_group: &Vec<u32>) {
    let mut ids: HashSet<u32> = HashSet::new();
    for v in left_group {
        ids.insert(*v);
    }
    let mut solution = 0;
    for v in right_group {
        if ids.contains(v) {
            solution += *v;
        }
    }
    println!("Day 1 Part 2: {}", solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    //TODO
}