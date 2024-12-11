use std::collections::HashSet;
use std::time::Instant;

const INPUT_FILE: &str = "input6.txt";

pub fn solve() {
    let mut start = Instant::now();
    let raw_input: Vec<String> = aoc_utils::input::load_input_lines(INPUT_FILE);
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut start_point: (char, usize, usize) = ('^', 0, 0);
    for (y, line) in raw_input.iter().enumerate() {
        let mut row: Vec<char> = Vec::new();
        for (x, c) in line.chars().enumerate() {
            if c == '^' {
                start_point = ('^', x, y)
            }
            row.push(c);
        }
        grid.push(row);
    }

    let setup_time = start.elapsed();
    println!("Setup: {:.2?}", setup_time);
    start = Instant::now();
    solve_part_one(&grid, start_point);
    let p1_time = start.elapsed();
    println!("Part 1: {:.2?}", setup_time + p1_time);
    start = Instant::now();
    solve_part_two(&mut grid, start_point);
    let p2_time = start.elapsed();
    println!("Part 2: {:.2?}", setup_time + p2_time);
}

fn solve_part_one(grid: &Vec<Vec<char>>, start: (char, usize, usize)) {
    let mut solution = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut cur = start;
    loop {
        visited.insert((cur.1, cur.2));
        let dir = cur.0;
        let x = cur.1;
        let y = cur.2;
        if dir == '^' {
            if y == 0 {
                break;
            } else if *(grid.get(y - 1).unwrap().get(x).unwrap()) == '#' {
                cur = ('>', x, y);
            } else {
                cur = ('^', x, y-1);
            }
        } else if dir == '>' {
            if x == grid.get(0).unwrap().len() - 1 {
                break;
            } else if *(grid.get(y).unwrap().get(x+1).unwrap()) == '#' {
                cur = ('v', x, y);
            } else {
                cur = ('>', x+1, y);
            }
        } else if dir == 'v' {
            if y == grid.len() - 1 {
                break;
            } else if *(grid.get(y + 1).unwrap().get(x).unwrap()) == '#' {
                cur = ('<', x, y);
            } else {
                cur = ('v', x, y + 1);
            } 
        } else if dir == '<' {
                if x == 0 {
                    break;
                } else if *(grid.get(y).unwrap().get(x-1).unwrap()) == '#' {
                    cur = ('^', x, y);
                } else {
                    cur = ('<', x-1, y);
                }
        }
    }
    println!("Day 6 Part 1: {}", visited.len());
}

fn solve_part_two(grid: &mut Vec<Vec<char>>, start: (char, usize, usize)) {
    let mut solution = 0;
    let mut states: HashSet<(char, usize, usize)> = HashSet::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut cur = start;

    for row in 0..grid.len() {
        for column in 0..grid.get(0).unwrap().len() {
            let candidate = grid.get(row).unwrap().get(column).unwrap();
            if *candidate != '.' {
                continue;
            }
            grid.get_mut(row).unwrap().push('#');
            let retained = grid.get_mut(row).unwrap().swap_remove(column);
            let mut found_cycle = false;
            cur = start;
            states.clear();
            visited.clear();
            loop {
                if states.contains(&cur) {
                    found_cycle = true;
                    break;
                }
                states.insert(cur);
                visited.insert((cur.1, cur.2));
                let dir = cur.0;
                let x = cur.1;
                let y = cur.2;
                if dir == '^' {
                    if y == 0 {
                        break;
                    } else if *(grid.get(y - 1).unwrap().get(x).unwrap()) == '#' {
                        cur = ('>', x, y);
                    } else {
                        cur = ('^', x, y-1);
                    }
                } else if dir == '>' {
                    if x == grid.get(0).unwrap().len() - 1 {
                        break;
                    } else if *(grid.get(y).unwrap().get(x+1).unwrap()) == '#' {
                        cur = ('v', x, y);
                    } else {
                        cur = ('>', x+1, y);
                    }
                } else if dir == 'v' {
                    if y == grid.len() - 1 {
                        break;
                    } else if *(grid.get(y + 1).unwrap().get(x).unwrap()) == '#' {
                        cur = ('<', x, y);
                    } else {
                        cur = ('v', x, y + 1);
                    } 
                } else if dir == '<' {
                        if x == 0 {
                            break;
                        } else if *(grid.get(y).unwrap().get(x-1).unwrap()) == '#' {
                            cur = ('^', x, y);
                        } else {
                            cur = ('<', x-1, y);
                        }
                }
            }
            if found_cycle {
                solution += 1;
            }
            grid.get_mut(row).unwrap().push(retained);
            grid.get_mut(row).unwrap().swap_remove(column);
        }
    }
    println!("Day 6 Part 2: {}", solution);
}

#[cfg(test)]
mod tests {
    use super::*;

}