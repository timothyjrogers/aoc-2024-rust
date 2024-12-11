use regex::Regex;
use std::time::Instant;

const INPUT_FILE: &str = "input3.txt";

pub fn solve() {
    let mut start = Instant::now();
    let raw_input: Vec<String> = aoc_utils::input::load_input_lines(INPUT_FILE);
    let input = raw_input.join("");

    let setup_time = start.elapsed();
    println!("Setup: {:.2?}", setup_time);
    start = Instant::now();
    solve_part_one(&input);
    let p1_time = start.elapsed();
    println!("Part 1: {:.2?}", setup_time + p1_time);
    start = Instant::now();
    solve_part_two(&input);
    let p2_time = start.elapsed();
    println!("Part 2: {:.2?}", setup_time + p2_time);
}

fn solve_part_one(input: &String) {
    let mut solution = 0;
    let muls = get_mul_matches(input);
    for mul in muls {
        solution += process_mul(&mul);
    }
    println!("Day 3 Part 1: {}", solution);
}


fn solve_part_two(input: &String) {
    let mut solution = 0;
    let mut enabled = true;
    let ops = get_mul_do_dont_matches(input);
    for op in ops {
        if op.starts_with("mul") && enabled {
            solution += process_mul(&op);
        } else if op.starts_with("don") {
            enabled = false;
        } else if op.starts_with("do") {
            enabled = true;
        }
        
    }
    println!("Day 3 Part 2: {}", solution);
}

fn get_mul_matches(line: &String) -> Vec<String> {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let matches: Vec<_> = re.find_iter(line).map(|m| m.as_str().to_owned()).collect();
    return matches;
}

fn get_mul_do_dont_matches(line: &String) -> Vec<String> {
    let re: Regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don\'t\(\)").unwrap();
    let matches: Vec<_> = re.find_iter(line).map(|m| m.as_str().to_owned()).collect();
    return matches;
}

fn process_mul(input: &String) -> u32 {
    let re = Regex::new(r"\d{1,3}").unwrap();
    let matches: Vec<_> = re.find_iter(input).map(|m| m.as_str().to_owned().parse::<u32>().unwrap()).collect();
    return matches.get(0).unwrap() * matches.get(1).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul_match_simple_pos() {
        assert_eq!(get_mul_matches(&String::from("mul(44,46)")), vec!["mul(44,46)"]);
    }

    #[test]
    fn test_mul_match_simple_neg() {
        assert_eq!(get_mul_matches(&String::from("mul(6,9!")).len(), 0);
    }

    #[test]
    fn test_mul_match_complex() {
        assert_eq!(get_mul_matches(&String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))")), vec!["mul(2,4)", "mul(5,5)", "mul(11,8)", "mul(8,5)"]);
    }
    
    #[test]
    fn test_process_mul() {
        let mul = String::from("mul(4,5)");
        assert_eq!(process_mul(&mul), 20);
    }

    #[test]
    fn test_mul_do_dont_match() {
        assert_eq!(get_mul_do_dont_matches(&String::from("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))")), vec!["mul(2,4)", "don't()", "mul(5,5)", "mul(11,8)", "do()", "mul(8,5)"])
    }
}