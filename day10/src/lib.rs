use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;

const INPUT_FILE: &str = "input10.txt";

pub fn solve() {
    let mut start = Instant::now();
    let raw_input: Vec<String> = aoc_utils::input::load_input_lines(INPUT_FILE);
    let x_max = raw_input.get(0).unwrap().len();
    let y_max = raw_input.len();
    let mut grid: HashMap<(usize, usize), u32> = HashMap::new();
    let mut trailheads: Vec<(usize, usize)> = Vec::new();
    for (y, row) in raw_input.iter().enumerate() {
        for (x, n) in row.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
            grid.insert((x, y), n);
            if n == 0 {
                trailheads.push((x,y));
            }
        }
    }

    let setup_time = start.elapsed();
    println!("Setup: {:.2?}", setup_time);
    start = Instant::now();
    solve_part_one(&grid, &trailheads, x_max, y_max);
    let p1_time = start.elapsed();
    println!("Part 1: {:.2?}", setup_time + p1_time);
    start = Instant::now();
    solve_part_two(&grid, &trailheads, x_max, y_max);
    let p2_time = start.elapsed();
    println!("Part 2: {:.2?}", setup_time + p2_time);
}

fn solve_part_one(grid: &HashMap<(usize, usize), u32>, trailheads: &Vec<(usize, usize)>, x_max: usize, y_max: usize) {
    let mut solution = 0;
    for trailhead in trailheads {
        let mut summits: HashSet<(usize, usize)> = HashSet::new();
        let mut stack: VecDeque<(usize, usize)> = VecDeque::new();
        stack.push_back(*trailhead);
        while stack.len() > 0 {
            let cur_pos = stack.pop_front().unwrap();
            let cur_val = grid.get(&cur_pos).unwrap();
            if *cur_val == 9 {
                summits.insert(cur_pos);
                continue;
            }
            for neighbor in get_neigbors(cur_pos, x_max, y_max) {
                let neighbor_val = grid.get(&neighbor).unwrap();
                if *neighbor_val > *cur_val && *neighbor_val - *cur_val == 1 {
                    stack.push_back(neighbor);
                }
            }
        }
        solution += summits.len();
    }
    println!("Day 9 Part 1: {}", solution);
}

fn solve_part_two(grid: &HashMap<(usize, usize), u32>, trailheads: &Vec<(usize, usize)>, x_max: usize, y_max: usize) {
    let mut solution = 0;
    for trailhead in trailheads {
        let mut summits: Vec<(usize, usize)> = Vec::new();
        let mut stack: VecDeque<(usize, usize)> = VecDeque::new();
        stack.push_back(*trailhead);
        while stack.len() > 0 {
            let cur_pos = stack.pop_front().unwrap();
            let cur_val = grid.get(&cur_pos).unwrap();
            if *cur_val == 9 {
                summits.push(cur_pos);
                continue;
            }
            for neighbor in get_neigbors(cur_pos, x_max, y_max) {
                let neighbor_val = grid.get(&neighbor).unwrap();
                if *neighbor_val > *cur_val && *neighbor_val - *cur_val == 1 {
                    stack.push_back(neighbor);
                }
            }
        }
        solution += summits.len();
    }
    println!("Day 9 Part 2: {}", solution);
}

fn get_neigbors(point: (usize, usize), x_max: usize, y_max: usize) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
        if point.0 > 0 {
            neighbors.push((point.0 - 1, point.1));
        }
        if point.0 < x_max - 1 {
            neighbors.push((point.0 + 1, point.1));
        }
        if point.1 > 0 {
            neighbors.push((point.0, point.1 - 1));
        }
        if point.1 < y_max - 1 {
            neighbors.push((point.0, point.1 + 1));
        }
    return neighbors;
}

#[cfg(test)]
mod tests {
    use super::*;

}