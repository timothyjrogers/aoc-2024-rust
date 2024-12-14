use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;

const INPUT_FILE: &str = "input12.txt";

enum Direction {
    N,
    E,
    S,
    W,
    NE,
    SE,
    SW,
    NW,
}

pub fn solve() {
    let mut start = Instant::now();
    let raw_input: Vec<String> = aoc_utils::input::load_input_lines(INPUT_FILE);
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in raw_input {
        let chars: Vec<char> = line.chars().collect();
        grid.push(chars);
    }
    let setup_time = start.elapsed();
    println!("Setup: {:.2?}", setup_time);
    start = Instant::now();

    solve_part_one(&grid);
    let p1_time = start.elapsed();
    println!("Part 1: {:.2?}", setup_time + p1_time);
    start = Instant::now();

    solve_part_two(&grid);
    let p2_time = start.elapsed();
    println!("Part 2: {:.2?}", setup_time + p2_time);
}

fn solve_part_one(grid: &Vec<Vec<char>>) {
    let mut solution = 0;
    let mut crops: HashMap<char, u64> = HashMap::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    for y_pos in 0..grid.len() {
        for x_pos in 0..grid.get(0).unwrap().len() {
            let starting_point = (Direction::N, (x_pos, y_pos));
            if !seen.contains(&starting_point.1) {
                let crop = grid.get(y_pos).unwrap().get(x_pos).unwrap();
                let mut cost_factors: (u64, u64) = (0, 0);
                let mut queue: VecDeque<(Direction, (usize, usize))> = VecDeque::new();
                queue.push_back(starting_point);
                while queue.len() > 0 {
                    let current = queue.pop_front().unwrap();
                    if seen.contains(&current.1) {
                        continue;
                    }
                    seen.insert(current.1);
                    let neighbors = get_neighbors(grid, current.1);
                    let mut perimiter_delta: u64 = 4 - neighbors.len() as u64;
                    for neighbor in neighbors {
                        let neighbor_crop = grid.get(neighbor.1.1).unwrap().get(neighbor.1.0).unwrap();
                        if neighbor_crop != crop {
                            perimiter_delta += 1;
                            continue;
                        }
                        if neighbor_crop == crop {
                            queue.push_back(neighbor);
                        }
                    }
                    cost_factors = (cost_factors.0 + 1, cost_factors.1 + perimiter_delta)
                }
                if crops.contains_key(&crop) {
                    let cost = crops.get(crop).unwrap();
                    crops.insert(*crop, cost + cost_factors.0 * cost_factors.1);
                } else {
                    crops.insert(*crop, cost_factors.0 * cost_factors.1);
                }
            }
        }
    }
    for (k, v) in crops {
        solution += v;
    }
    println!("Day 12 Part 1: {}", solution);
}

fn solve_part_two(grid: &Vec<Vec<char>>) {
    let mut solution = 0;
    let mut regions: Vec<HashSet<(usize,usize)>> = Vec::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    for y_pos in 0..grid.len() {
        for x_pos in 0..grid.get(0).unwrap().len() {
            let starting_point = (Direction::N, (x_pos, y_pos)); // arbitrary direction for starting point
            if !seen.contains(&starting_point.1) {
                let mut region: HashSet<(usize, usize)> = HashSet::new();
                let mut corners: usize = 0;
                let crop = grid.get(y_pos).unwrap().get(x_pos).unwrap();
                let mut queue: VecDeque<(Direction, (usize, usize))> = VecDeque::new();
                queue.push_back(starting_point);
                while queue.len() > 0 {
                    let current = queue.pop_front().unwrap();
                    if seen.contains(&current.1) {
                        continue;
                    }
                    region.insert(current.1);
                    seen.insert(current.1);
                    let neighbors = get_neighbors(grid, current.1);
                    let mut north = false;
                    let mut east = false;
                    let mut south = false;
                    let mut west = false;
                    let mut matching_neighbors = 0;
                    for neighbor in &neighbors {
                        let neighbor_crop = grid.get(neighbor.1.1).unwrap().get(neighbor.1.0).unwrap();
                        if neighbor_crop == crop {
                            matching_neighbors += 1;
                        }
                        match neighbor.0 {
                            Direction::N => north = neighbor_crop == crop,
                            Direction::E => east = neighbor_crop == crop,
                            Direction::S => south = neighbor_crop == crop,
                            Direction::W => west = neighbor_crop == crop,
                            _ => (),
                        }
                    }
                    if matching_neighbors == 0 {
                        corners += 4;
                    } else if matching_neighbors == 1 {
                        corners += 2;
                    } else if matching_neighbors == 2 {
                        let mut corner_count = 0;
                        if (north && east) || (south && east) || (south && west) || (north && west) {
                            corner_count = 1
                        }
                        if north && east && grid.get(current.1.1 - 1).unwrap().get(current.1.0 + 1).unwrap() != crop {
                            corner_count += 1;
                        }
                        if south && east && grid.get(current.1.1 + 1).unwrap().get(current.1.0 + 1).unwrap() != crop {
                            corner_count += 1;
                        }
                        if south && west && grid.get(current.1.1 + 1).unwrap().get(current.1.0 - 1).unwrap() != crop {
                            corner_count += 1;
                        }
                        if north && west && grid.get(current.1.1 - 1).unwrap().get(current.1.0 - 1).unwrap() != crop {
                            corner_count += 1;
                        }
                        corners += corner_count;
                    } else {
                        let mut corner_count = 0;
                        if north && east && grid.get(current.1.1 - 1).unwrap().get(current.1.0 + 1).unwrap() != crop {
                            corner_count += 1;
                        }
                        if south && east && grid.get(current.1.1 + 1).unwrap().get(current.1.0 + 1).unwrap() != crop {
                            corner_count += 1;
                        }
                        if south && west && grid.get(current.1.1 + 1).unwrap().get(current.1.0 - 1).unwrap() != crop {
                            corner_count += 1;
                        }
                        if north && west && grid.get(current.1.1 - 1).unwrap().get(current.1.0 - 1).unwrap() != crop {
                            corner_count += 1;
                        }
                        corners += corner_count;
                    }
                    for neighbor in neighbors {
                        let neighbor_crop = grid.get(neighbor.1.1).unwrap().get(neighbor.1.0).unwrap();
                        if neighbor_crop != crop {
                            continue;
                        }
                        if neighbor_crop == crop {
                            queue.push_back(neighbor);
                        }
                    }
                }
                // End BFS for this starting point
                println!("Crop: {}, Area: {}, Sides: {}", crop, region.len(), corners);
                solution += corners * region.len();
            }
        }
    }
    println!("Day 12 Part 2: {}", solution);
}

fn get_neighbors(grid: &Vec<Vec<char>>, points: (usize, usize)) -> Vec<(Direction, (usize, usize))> {
    let mut neighbors: Vec<(Direction, (usize, usize))> = Vec::new();
    if points.0 > 0 {
        neighbors.push((Direction::W, (points.0 - 1, points.1)));
    }
    if points.1 > 0 {
        neighbors.push((Direction::N, (points.0, points.1 - 1)));
    }
    if points.0 < grid.get(0).unwrap().len() - 1 {
        neighbors.push((Direction::E, (points.0 + 1, points.1)));
    }
    if points.1 < grid.len() - 1 {
        neighbors.push((Direction::S, (points.0, points.1 + 1)));
    }
    return neighbors;
}

#[cfg(test)]
mod tests {
    use super::*;

}