use regex::Regex;
use std::time::Instant;

const INPUT_FILE: &str = "input13.txt";

struct TwoEquationSystem {
    a1: i64,
    b1: i64,
    c1: i64,
    a2: i64,
    b2: i64,
    c2: i64
}

impl TwoEquationSystem {
    fn new(a1: i64, b1: i64, c1: i64, a2: i64, b2: i64, c2: i64) -> Self {
        Self { a1, b1, c1, a2, b2, c2 }
    }

    fn solve(&self) -> Option<(i64, i64)> {
        // Cramer's rule: https://en.wikipedia.org/wiki/Cramer's_rule
        let coefficient_det = self.a1 * self.b2 - self.b1 * self.a2;
        let x_numer_det = self.c1 * self.b2 - self.b1 * self.c2;
        let y_numer_det = self.a1 * self.c2 - self.c1 * self.a2;

        if x_numer_det % coefficient_det != 0 || y_numer_det % coefficient_det != 0 {
            return None;
        }
        let x = x_numer_det / coefficient_det;
        let y = y_numer_det / coefficient_det;
        return Some((x,y));
    }
}

pub fn solve() {
    let mut start = Instant::now();
    let raw_input: Vec<String> = aoc_utils::input::load_input_lines(INPUT_FILE);
    let mut index: usize = 0;
    let mut machines: Vec<TwoEquationSystem> = Vec::new();
    while index < raw_input.len() {
        let a_line = raw_input.get(index).unwrap();
        let b_line = raw_input.get(index + 1).unwrap();
        let prize_line = raw_input.get(index + 2).unwrap();
        index += 4; // skip the empty line between machines
        let re = Regex::new(r"\d+").unwrap();
        let a_matches: Vec<_> = re.find_iter(a_line).map(|m| m.as_str().parse::<i64>().unwrap()).collect();
        let b_matches: Vec<_> = re.find_iter(b_line).map(|m| m.as_str().parse::<i64>().unwrap()).collect();
        let prize_matches: Vec<_> = re.find_iter(prize_line).map(|m| m.as_str().parse::<i64>().unwrap()).collect();
        machines.push(TwoEquationSystem::new(*a_matches.get(0).unwrap(), *b_matches.get(0).unwrap(), *prize_matches.get(0).unwrap(), *a_matches.get(1).unwrap(), *b_matches.get(1).unwrap(), *prize_matches.get(1).unwrap()));
    }
    let setup_time = start.elapsed();
    println!("Setup: {:.2?}", setup_time);
    start = Instant::now();
    solve_part_one(&machines);
    let p1_time = start.elapsed();
    println!("Part 1: {:.2?}", setup_time + p1_time);
    start = Instant::now();

    for machine in &mut machines {
        machine.c1 += 10000000000000;
        machine.c2 += 10000000000000;
    }
    solve_part_two(&machines);
    let p2_time = start.elapsed();
    println!("Part 2: {:.2?}", setup_time + p2_time);
}

fn solve_part_one(machines: &Vec<TwoEquationSystem>) {
    let mut solution = 0;
    for (index, machine) in machines.iter().enumerate() {
        match machine.solve() {
            Some(v) => solution += 3 * v.0 + v.1,
            None => (),
        }
    }
    println!("Day 11 Part 1: {}", solution);
}

fn solve_part_two(machines: &Vec<TwoEquationSystem>) {
    let mut solution = 0;
    for (index, machine) in machines.iter().enumerate() {
        match machine.solve() {
            Some(v) => solution += 3 * v.0 + v.1,
            None => (),
        }
    }
    println!("Day 11 Part 2: {}", solution);
}

#[cfg(test)]
mod tests {
    use super::*;

}