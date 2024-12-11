use std::collections::VecDeque;
use std::time::Instant;

const INPUT_FILE: &str = "input7.txt";

pub fn solve() {
    let mut start = Instant::now();
    let raw_input: Vec<String> = aoc_utils::input::load_input_lines(INPUT_FILE);
    let mut cases: Vec<(u64, VecDeque<u64>)> = Vec::new();
    for line in raw_input {
        let parts: Vec<&str> = line.split(": ").collect();
        let target = parts.get(0).unwrap().parse::<u64>().unwrap();
        let values: VecDeque<u64> = parts.get(1).unwrap().split(" ").map(|s| s.parse::<u64>().unwrap()).collect();
        cases.push((target, values));
    }

    let setup_time = start.elapsed();
    println!("Setup: {:.2?}", setup_time);
    start = Instant::now();
    solve_part_one(cases.clone());
    let p1_time = start.elapsed();
    println!("Part 1: {:.2?}", setup_time + p1_time);
    start = Instant::now();
    solve_part_two(cases.clone());
    let p2_time = start.elapsed();
    println!("Part 2: {:.2?}", setup_time + p2_time);
}

fn solve_part_one(cases: Vec<(u64, VecDeque<u64>)>) {
    let mut solution = 0;
    for case in cases {
        let target = case.0;
        let values = case.1;
        if process_equation(values, target) {
            solution += target;
        }
    }
    println!("Day 7 Part 1: {}", solution);
}

fn solve_part_two(cases: Vec<(u64, VecDeque<u64>)>) {
    let mut solution = 0;
    for case in cases {
        let target = case.0;
        let values = case.1;
        if process_equation_with_pipes(values, target) {
            solution += target;
        }
    }
    println!("Day 7 Part 2: {}", solution);
}

fn process_equation(values: VecDeque<u64>, target: u64) -> bool {
    let mut values = values.clone();
    if values.len() == 1 {
        let result = values.pop_front().unwrap();
        return result == target
    }
    let current = values.pop_front().unwrap();
    let next = values.pop_front().unwrap();
    let mut add = values.clone();
    add.push_front(current + next);
    let mut mul = values.clone();
    mul.push_front(current * next);
    if process_equation(add, target) || process_equation(mul, target)  {
        return true;
    }
    return false;
}

fn process_equation_with_pipes(values: VecDeque<u64>, target: u64) -> bool {
    let mut values = values.clone();
    if values.len() == 1 {
        let result = values.pop_front().unwrap();
        return result == target
    }
    let current = values.pop_front().unwrap();
    let next = values.pop_front().unwrap();
    let mut add = values.clone();
    add.push_front(current + next);
    let mut mul = values.clone();
    mul.push_front(current * next);
    let mut pipes = values.clone();
    let mut current_string: String = current.to_string();
    let next_string: String = next.to_string();
    current_string.push_str(next_string.as_str());
    pipes.push_front(current_string.parse::<u64>().unwrap());
    if process_equation_with_pipes(add, target) || process_equation_with_pipes(mul, target) || process_equation_with_pipes(pipes, target)  {
        return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

}