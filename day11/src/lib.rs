use std::collections::HashMap;
use std::time::Instant;

const INPUT_FILE: &str = "input11.txt";

pub fn solve() {
    let mut start = Instant::now();
    let raw_input: String = aoc_utils::input::load_input_line(INPUT_FILE);
    let mut input: Vec<u64> = raw_input.split(" ").map(|s| s.parse::<u64>().unwrap()).collect();
    let mut hashed_input: HashMap<u64, u64>  = HashMap::new();
    for stone in raw_input.split(" ").map(|s| s.parse::<u64>().unwrap()) {
        hashed_input.entry(stone).or_insert(1);
    }

    let setup_time = start.elapsed();
    println!("Setup: {:.2?}", setup_time);
    start = Instant::now();
    solve_part_one(&mut input);
    let p1_time = start.elapsed();
    println!("Part 1: {:.2?}", setup_time + p1_time);
    start = Instant::now();

    solve_part_two(hashed_input);
    let p2_time = start.elapsed();
    println!("Part 2: {:.2?}", setup_time + p2_time);
}

fn solve_part_one(input: &mut Vec<u64>) {
    for _ in 0..25 {
        let mut index = 0;
        while index < input.len() {
            let cur = input.remove(index);
            if cur == 0 {
                input.insert(index, 1);
                index += 1;
            } else if cur.to_string().len() %2 == 0 {
                let mut cur_str = cur.to_string();
                let back_half = cur_str.split_off(cur_str.len() / 2);
                input.insert(index, cur_str.parse::<u64>().unwrap());
                if index == input.len() - 1 {
                    input.push(back_half.parse::<u64>().unwrap());
                } else {
                    input.insert(index + 1, back_half.parse::<u64>().unwrap());
                }
                index += 2;
            } else {
                input.insert(index, cur * 2024);
                index += 1;
            }
        }
    }
    println!("Day 11 Part 1: {}", input.len());
}

fn solve_part_two(input: HashMap<u64, u64>) {
    let mut solution = 0;
    let mut current_map = input; 
    for _ in 0..75 {
        let mut new_map: HashMap<u64,u64> = HashMap::new();
        for(number, count) in current_map {
            if number == 0 {
                *new_map.entry(1).or_default() += count;
            } else if number.to_string().len() %2 == 0 {
                let mut cur_str = number.to_string();
                let back_half = cur_str.split_off(cur_str.len() / 2);
                *new_map.entry(cur_str.parse::<u64>().unwrap()).or_default() += count;
                *new_map.entry(back_half.parse::<u64>().unwrap()).or_default() += count;
            } else {
                *new_map.entry(number * 2024).or_default() += count;
            }
        }
        current_map = new_map;
    }
    for (k, v) in current_map {
        solution += v;
    }
    println!("Day 11 Part 2: {}", solution);
}

#[cfg(test)]
mod tests {
    use super::*;

}