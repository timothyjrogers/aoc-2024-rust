use std::collections::HashMap;
use std::time::Instant;

const INPUT_FILE: &str = "input5.txt";

pub fn solve() {
    let mut start = Instant::now();
    let raw_input: Vec<String> = aoc_utils::input::load_input_lines(INPUT_FILE);

    let setup_time = start.elapsed();
    println!("Setup: {:.2?}", setup_time);
    start = Instant::now();
    solve_part_one(&raw_input);
    let p1_time = start.elapsed();
    println!("Part 1: {:.2?}", setup_time + p1_time);
    start = Instant::now();
    solve_part_two(&raw_input);
    let p2_time = start.elapsed();
    println!("Part 2: {:.2?}", setup_time + p2_time);
}

fn solve_part_one(input: &Vec<String>) {
    let mut solution = 0;

    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut updates: Vec<Vec<String>> = Vec::new();
    let mut stage = 0;
    for line in input {
        if line == "" {
            stage = 1;
            continue;
        }
        if stage == 0 {
            let parts = parse_rule(line);
            if rules.contains_key(&parts.0) {
                rules.get_mut(&parts.0).unwrap().push(parts.1);
            } else {
                rules.insert(parts.0, vec![parts.1]);
            }
        } else {
            let pages: Vec<u32> = parse_pages(line);
            if validate_update(&rules, &pages) {
                solution += pages.get(pages.len()/2).unwrap();
            }
        }
    }
    println!("Day 5 Part 1: {}", solution);
}

fn solve_part_two(input: &Vec<String>) {
    let mut solution = 0;
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut updates: Vec<Vec<String>> = Vec::new();
    let mut invalid: Vec<Vec<u32>> = Vec::new();
    let mut stage = 0;

    for line in input {
        if line == "" {
            stage = 1;
            continue;
        }
        if stage == 0 {
            let parts = parse_rule(line);
            if rules.contains_key(&parts.0) {
                rules.get_mut(&parts.0).unwrap().push(parts.1);
            } else {
                rules.insert(parts.0, vec![parts.1]);
            }
        } else {
            let pages: Vec<u32> = parse_pages(line);
            if !validate_update(&rules, &pages) {
                invalid.push(pages);
            }
        }
    }
    for update in invalid {
        let mut cur = update.clone();
        while !validate_update(&rules, &cur) {
            let mut changed = false;
            for i in 0..cur.len() {
                let c = cur.get(i).unwrap();
                for j in i+1..cur.len() {
                    let n = cur.get(j).unwrap();
                    match rules.get(n) {
                        Some(v) => {
                            if v.contains(c) {
                                cur.swap(i,j);
                                changed = true;
                                break;
                            }
                        },
                        _ => ()
                    }
                }
                if changed {
                    break;
                }
            }
        }
        solution += cur.get(cur.len()/2).unwrap();
    }
    println!("Day 5 Part 2: {}", solution);
}

fn parse_rule(input: &String) -> (u32, u32) {
    let parts: Vec<u32> = input.split("|").map(|s| s.parse::<u32>().unwrap()).collect();
    return (*parts.get(0).unwrap(), *parts.get(1).unwrap());
}

fn parse_pages(input: &String) -> Vec<u32> {
    let parts = input.split(",").map(|s| s.parse::<u32>().unwrap()).collect();
    return parts;
}

fn validate_update(rules: &HashMap<u32, Vec<u32>>, update: &Vec<u32>) -> bool {
    let mut valid = true;
    for i in 0..update.len() {
        let cur = update.get(i).unwrap();
        for j in i+1..update.len() {
            let next = update.get(j).unwrap();
            match rules.get(next) {
                Some(v) => {
                    if v.contains(cur) {
                        valid = false;
                        break;
                    }
                },
                _ => ()
            }
        }
        if !valid {
            break;
        }
    }
    return valid;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rule() {
        let rule = String::from("99|75");
        let expected: (u32, u32) = (99, 75);
        assert_eq!(parse_rule(&rule), expected);
    }

    #[test]
    fn test_parse_pages() {
        let rule = String::from("1,2,3,4,5");
        let expected: Vec<u32> = vec![1,2,3,4,5];
        assert_eq!(parse_pages(&rule), expected);
    }

    #[test]
    fn test_validate_update_positive() {
        let rules: HashMap<u32, Vec<u32>> = HashMap::from([(99, vec![75])]);
        let update: Vec<u32> = vec![1,2,3,95,75];
        assert_eq!(validate_update(&rules, &update), true);
    }

    #[test]
    fn test_validate_update_negative() {
        let rules: HashMap<u32, Vec<u32>> = HashMap::from([(99, vec![75])]);
        let update: Vec<u32> = vec![1,2,3,75,99];
        assert_eq!(validate_update(&rules, &update), false);
    }
}