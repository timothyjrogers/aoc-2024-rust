use std::collections::{HashMap, HashSet};
use std::cmp::{min, max};
use std::time::Instant;

const INPUT_FILE: &str = "input8.txt";

pub fn solve() {
    let mut start = Instant::now();
    let raw_input: Vec<String> = aoc_utils::input::load_input_lines(INPUT_FILE);
    //let grid: HashMap<(isize,isize), char> = HashMap::new();
    let x_max = raw_input.get(0).unwrap().len() as isize;
    let y_max = raw_input.len() as isize;
    let mut antennas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    for (row, line) in raw_input.iter().enumerate() {
        for (pos, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            let coordinate = (pos as isize, row as isize);
            let value = c;
            if antennas.contains_key(&c) {
                antennas.get_mut(&c).unwrap().push(coordinate);
            } else {
                antennas.insert(c, vec![coordinate]);
            }
        }
    }

    let setup_time = start.elapsed();
    println!("Setup: {:.2?}", setup_time);
    start = Instant::now();
    solve_part_one(&antennas, x_max, y_max);
    let p1_time = start.elapsed();
    println!("Part 1: {:.2?}", setup_time + p1_time);
    start = Instant::now();
    solve_part_two(&antennas, x_max, y_max);
    let p2_time = start.elapsed();
    println!("Part 2: {:.2?}", setup_time + p2_time);
}

fn solve_part_one(antennas: &HashMap<char, Vec<(isize, isize)>>, x_max: isize, y_max: isize) {
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
    for (key, value) in antennas {
        if value.len() < 2 {
            continue;
        }
        for i1 in 0..value.len()-1 {
            for i2 in i1+1..value.len() {
                let p1 = value.get(i1).unwrap();
                let p2 = value.get(i2).unwrap();
                let dx = p1.0.abs_diff(p2.0) as isize;
                let dy = p1.1.abs_diff(p2.1) as isize;
                if p1.0 <= p2.0 && p1.1 <= p2.1 { // p1 is up-left from p2
                    if p1.0 >= dx && p1.1 >= dy {
                        antinodes.insert((p1.0 - dx, p1.1 - dy));
                    }
                    if p2.0 <= x_max - dx - 1 && p2.1 <= y_max - dy - 1 {
                        antinodes.insert((p2.0 + dx, p2.1 + dy));
                    }
                } else if p1.0 <= p2.0 && p1.1 > p2.1 { // p1 is down-left from p2
                    if p1.0 >= dx && p1.1 <= y_max - dy -1 {
                        antinodes.insert((p1.0 - dx, p1.1 + dy));
                    }
                    if p2.0 <= x_max - dx - 1 && p2.1 >= dy {
                        antinodes.insert((p2.0 + dx, p2.1 - dy));
                    }
                } else if p1.0 > p2.0 && p1.1 <= p2.1 { // p1 is up-right from p2
                    if p1.0 <= x_max - dx - 1 && p1.1 >= dy {
                        antinodes.insert((p1.0 + dx, p1.1 - dy));
                    }
                    if p2.0 >= dx && p2.1 <= y_max - dy - 1 {
                        antinodes.insert((p2.0 - dx, p2.1 + dy));
                    }
                } else if p1.0 > p2.0 && p1.1 > p2.1 { // p1 is down-right from p2
                    if p1.0 <= x_max - dx - 1 && p1.1 <= y_max - dy - 1 {
                        antinodes.insert((p1.0 + dx, p1.1 + dy));
                    }
                    if p2.0 >= dx && p2.1 >= dy {
                        antinodes.insert((p2.0 - dx, p2.1 - dy));
                    }
                }
            }
        }
    }
    println!("Day 8 Part 1: {}", antinodes.len());
}



fn solve_part_two(antennas: &HashMap<char, Vec<(isize, isize)>>, x_max: isize, y_max: isize) {
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
    for (key, value) in antennas {
        if value.len() < 2 {
            continue;
        }
        for i1 in 0..value.len()-1 {
            for i2 in i1+1..value.len() {
                let p1 = value.get(i1).unwrap();
                let p2 = value.get(i2).unwrap();
                let dx = p1.0.abs_diff(p2.0) as isize;
                let dy = p1.1.abs_diff(p2.1) as isize;
                antinodes.insert(*p1);
                antinodes.insert(*p2);
                if p1.0 <= p2.0 && p1.1 <= p2.1 { // p1 is up-left from p2
                    let mut p1_tmp = (p1.0 - dx, p1.1 - dy);
                    while p1_tmp.0 >= 0 && p1_tmp.1 >= 0 {
                        antinodes.insert(p1_tmp);
                        p1_tmp = (p1_tmp.0 - dx, p1_tmp.1 - dy);
                    }
                    let mut p2_tmp = (p2.0 + dx, p2.1 + dy);
                    while p2_tmp.0 <= x_max - 1 && p2_tmp.1 <= y_max - 1 {
                        antinodes.insert(p2_tmp);
                        p2_tmp = (p2_tmp.0 + dx, p2_tmp.1 + dy);
                    }
                } else if p1.0 <= p2.0 && p1.1 > p2.1 { // p1 is down-left from p2
                    let mut p1_tmp = (p1.0 - dx, p1.1 + dy);
                    while p1_tmp.0 >= 0 && p1_tmp.1 <= y_max - 1 {
                        antinodes.insert(p1_tmp);
                        p1_tmp = (p1_tmp.0 - dx, p1_tmp.1 + dy);
                    }
                    let mut p2_tmp = (p2.0 + dx, p2.1 - dy);
                    while p2_tmp.0 <= x_max - 1 && p2_tmp.1 <= 0 {
                        antinodes.insert(p2_tmp);
                        p2_tmp = (p2_tmp.0 + dx, p2_tmp.1 - dy);
                    }
                } else if p1.0 > p2.0 && p1.1 <= p2.1 { // p1 is up-right from p2
                    let mut p1_tmp = (p1.0 + dx, p1.1 - dy);
                    while p1_tmp.0 <= x_max - 1 && p1_tmp.1 >= 0 {
                        antinodes.insert(p1_tmp);
                        p1_tmp = (p1_tmp.0 + dx, p1_tmp.1 - dy);
                    }
                    let mut p2_tmp = (p2.0 - dx, p2.1 + dy);
                    while p2_tmp.0 >= 0 && p2_tmp.1 <= y_max - 1 {
                        antinodes.insert(p2_tmp);
                        p2_tmp = (p2_tmp.0 - dx, p2_tmp.1 + dy);
                    }
                } else if p1.0 > p2.0 && p1.1 > p2.1 { // p1 is down-right from p2
                    let mut p1_tmp = (p1.0 + dx, p1.1 + dy);
                    while p1_tmp.0 <= x_max - 1 && p1_tmp.1 <= y_max - 1 {
                        antinodes.insert(p1_tmp);
                        p1_tmp = (p1_tmp.0 + dx, p1_tmp.1 + dy);
                    }
                    let mut p2_tmp = (p2.0 - dx, p2.1 - dy);
                    while p2_tmp.0 >= 0 && p2_tmp.1 >= 0 {
                        antinodes.insert(p2_tmp);
                        p2_tmp = (p2_tmp.0 - dx, p2_tmp.1 - dy);
                    }
                }
            }
        }
    }
    println!("Day 8 Part 2: {}", antinodes.len());
}


#[cfg(test)]
mod tests {
    use super::*;

}