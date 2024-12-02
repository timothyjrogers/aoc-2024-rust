use std::collections::HashSet;

const INPUT_FILE: &str = "input2.txt";

pub fn solve() {
    let raw_input: Vec<String> = aoc_utils::input::load_input_lines(INPUT_FILE);
    let mut reports: Vec<Vec<u32>> = Vec::new();
    for line in raw_input {
        let parts = line.split(" ");
        let report: Vec<u32> = parts.map(|x| x.parse::<u32>().unwrap()).collect();
        reports.push(report);
    }

    solve_part_one(&reports);
    solve_part_two(&reports);
}

fn solve_part_one(reports: &Vec<Vec<u32>>) {
    let mut solution = 0;
    for report in reports {
        if is_valid(report) {
            solution += 1;
        }
    }
    println!("Day 2 Part 1: {}", solution);
}


fn solve_part_two(reports: &Vec<Vec<u32>>) {
    let mut solution = 0;
    for report in reports {
        for i in 0..report.len() {
            let mut current = report.clone();
            current.remove(i);
            if is_valid(&current) {
                solution += 1;
                break;
            }
        }
    }
    println!("Day 2 Part 2: {}", solution);
}


fn is_valid(report: &Vec<u32>) -> bool {
    let decreasing = report.get(0).unwrap() > report.get(1).unwrap();
    for i in 0..report.len()-1 {
        let cur = report.get(i).unwrap();
        let next = report.get(i+1).unwrap();
        if decreasing && next > cur {
            return false;
        }
        if !decreasing && next < cur {
            return false;
        }
        if cur == next {
            return false;
        }
        if next.abs_diff(*cur) < 1 || next.abs_diff(*cur) > 3 {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    //TODO
}