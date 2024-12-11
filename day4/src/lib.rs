use std::collections::HashMap;
use std::time::Instant;

const INPUT_FILE: &str = "input4.txt";

pub fn solve() {
    let mut start = Instant::now();
    let raw_input: Vec<String> = aoc_utils::input::load_input_lines(INPUT_FILE);
    let y_max = raw_input.len();
    let x_max = raw_input.get(0).unwrap().len();
    let points = map_char_points(raw_input);

    let setup_time = start.elapsed();
    println!("Setup: {:.2?}", setup_time);
    start = Instant::now();
    solve_part_one(&points, x_max, y_max);
    let p1_time = start.elapsed();
    println!("Part 1: {:.2?}", setup_time + p1_time);
    start = Instant::now();
    solve_part_two(&points, x_max, y_max);
    let p2_time = start.elapsed();
    println!("Part 2: {:.2?}", setup_time + p2_time);
}

fn solve_part_one(points: &HashMap<(usize,usize), char>, x_max: usize, y_max: usize) {
    let mut solution = 0;
    for (key, val) in points.iter() {
        if *val == 'X' {
            let paths_to_check = get_paths(key, x_max, y_max);
            for path in paths_to_check {
                let mut valid = true;
                match points.get(path.get(0).unwrap()) {
                    Some(n) => {
                        if *n != 'M' {
                            valid = false;
                        }
                    },
                    _ => ()
                }
                match points.get(path.get(1).unwrap()) {
                    Some(n) => {
                        if *n != 'A' {
                            valid = false;
                        }
                    },
                    _ => ()
                }
                match points.get(path.get(2).unwrap()) {
                    Some(n) => {
                        if *n != 'S' {
                            valid = false;
                        }
                    },
                    _ => ()
                }
                if valid {
                    solution += 1;
                }
            }
        }
    }
    println!("Day 4 Part 1: {}", solution);
}


fn solve_part_two(points: &HashMap<(usize,usize), char>, x_max: usize, y_max: usize) {
    let mut solution = 0;
    for (key, val) in points.iter() {
        if *val == 'A' {
            let points_to_check = get_corners(key, x_max, y_max);
            if points_to_check.len() < 4 {
                continue;
            }
            let up_left = points.get(points_to_check.get(0).unwrap()).unwrap();
            let up_right = points.get(points_to_check.get(1).unwrap()).unwrap();
            let down_left = points.get(points_to_check.get(2).unwrap()).unwrap();
            let down_right = points.get(points_to_check.get(3).unwrap()).unwrap();

            if (*up_left != 'M' && *up_left != 'S') || (*up_right != 'M' && *up_right != 'S') || (*down_left != 'M' && *down_left != 'S') || (*down_right != 'M' && *down_right != 'S') {
                continue;
            }

            if *up_left == 'M' {
                if *down_right == 'S' && ((*up_right == 'M' && *down_left == 'S') || (*up_right == 'S' && *down_left == 'M')) {
                    solution += 1;
                }
            } else if *up_left == 'S' {
                if *down_right == 'M' && ((*up_right == 'M' && *down_left == 'S') || (*up_right == 'S' && *down_left == 'M')) {
                    solution += 1;
                }
            }
        }
    }
    println!("Day 4 Part 2: {}", solution);
}

fn map_char_points(input: Vec<String>) -> HashMap<(usize,usize), char> {
    let mut char_points: HashMap<(usize, usize), char> = HashMap::new();
    for y in 0..input.len() {
        for (x, c) in input.get(y).unwrap().chars().enumerate() {
            let point: (usize, usize) = (x, y);
            if c == 'X' || c == 'M' || c == 'A' || c == 'S' {
                char_points.insert(point, c);
            }
        }
    }
    return char_points;
}

fn get_paths(position: &(usize, usize), x_max: usize, y_max: usize) -> Vec<Vec<(usize, usize)>> {
    let mut paths: Vec<Vec<(usize,usize)>> = Vec::new();

    let include_up_paths = position.1 >= 3;
    let include_down_paths = y_max - position.1 - 1 >= 3;
    let include_left_paths = position.0 >= 3;
    let include_right_paths = x_max - position.0 - 1 >= 3;

    if include_up_paths {
        paths.push(vec![(position.0, position.1 - 1), (position.0, position.1 - 2), (position.0, position.1 - 3)]);
        if include_left_paths {
            paths.push(vec![(position.0 - 1, position.1 - 1), (position.0 - 2, position.1 - 2), (position.0 - 3, position.1 - 3)]);
        }
        if include_right_paths {
            paths.push(vec![(position.0 + 1, position.1 - 1), (position.0 + 2, position.1 - 2), (position.0 + 3, position.1 - 3)]);
        }
    }
    if include_left_paths {
        paths.push(vec![(position.0 - 1, position.1), (position.0 - 2, position.1), (position.0 - 3, position.1)]);
    }
    if include_right_paths {
        paths.push(vec![(position.0 + 1, position.1), (position.0 + 2, position.1), (position.0 + 3, position.1)]);
    }
    if include_down_paths {
        paths.push(vec![(position.0 , position.1 + 1), (position.0 , position.1 + 2), (position.0 , position.1 + 3)]);
        if include_left_paths {
            paths.push(vec![(position.0 - 1, position.1 + 1), (position.0 - 2, position.1 + 2), (position.0 - 3, position.1 + 3)]);
        }
        if include_right_paths {
            paths.push(vec![(position.0 + 1, position.1 + 1), (position.0 + 2, position.1 + 2), (position.0 + 3, position.1 + 3)]);
        }
    }
    return paths;
}

fn get_corners(position: &(usize, usize), x_max: usize, y_max: usize) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = Vec::new();

    let include_up = position.1 >= 1;
    let include_down = y_max - position.1 - 1 >= 1;
    let include_left = position.0 >= 1;
    let include_right = x_max - position.0 - 1 >= 1;

    if include_up && include_left {
        neighbors.push((position.0 - 1, position.1 - 1));
    }
    if include_up && include_right {
        neighbors.push((position.0 + 1, position.1 - 1));
    }
    if include_down && include_left {
        neighbors.push((position.0 - 1, position.1 + 1));
    }
    if include_down && include_right {
        neighbors.push((position.0 + 1, position.1 + 1))
    }
    return neighbors;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_char_points() {
        let mut input = Vec::new();
        input.push(String::from("..X..."));
        input.push(String::from(".SAMX."));
        input.push(String::from(".A..A."));
        input.push(String::from("XMAS.S"));
        input.push(String::from(".X...."));

        let mut expected_result: HashMap<(usize, usize), char> = HashMap::new();
        expected_result.insert((2,0), 'X');
        expected_result.insert((1,1), 'S');
        expected_result.insert((2,1), 'A');
        expected_result.insert((3,1), 'M');
        expected_result.insert((4,1), 'X');
        expected_result.insert((1,2), 'A');
        expected_result.insert((4,2), 'A');
        expected_result.insert((0,3), 'X');
        expected_result.insert((1,3), 'M');
        expected_result.insert((2,3), 'A');
        expected_result.insert((3,3), 'S');
        expected_result.insert((5,3), 'S');
        expected_result.insert((1,4), 'X');
        
        assert_eq!(map_char_points(input), expected_result);
    }

    #[test]
    fn test_get_corners_all() {
        let position = (1,1);
        let x_max = 10;
        let y_max = 10;
        assert_eq!(get_corners(&position, x_max, y_max), vec![(0,0), (2,0), (0,2), (2,2)]);
    }
}