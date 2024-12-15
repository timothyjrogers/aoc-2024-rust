use regex::Regex;
use std::time::Instant;

const INPUT_FILE: &str = "input14.txt";
const X_MAX: isize = 101;
const Y_MAX: isize = 103;

enum Quadrant {
    NONE,
    ONE,
    TWO,
    THREE,
    FOUR,
}

#[derive(Clone)]
struct Bot {
    px: isize,
    py: isize,
    vx: isize,
    vy: isize,
}

impl Bot {
    fn new(px: isize, py: isize, vx: isize, vy: isize) -> Self {
        Self { px, py, vx, vy }
    }

    fn tick_n_times(&mut self, n: isize) {
        self.px = (self.px + n * self.vx).rem_euclid(X_MAX);
        self.py = (self.py + n * self.vy).rem_euclid(Y_MAX);
    }

    fn get_quadrant(&self) -> Quadrant {
        let mid_x = X_MAX/2;
        let mid_y = Y_MAX/2;
        if self.px < mid_x && self.py < mid_y { 
            return Quadrant::ONE;
        }
        if self.px > mid_x && self.py < mid_y {
            return Quadrant::TWO;
        }
        if self.px < mid_x && self.py > mid_y {
            return Quadrant::THREE;
        }
        if self.px > mid_x && self.py > mid_y {
            return Quadrant::FOUR;
        }
        return Quadrant::NONE;
    }
}

pub fn solve() {
    let mut start = Instant::now();
    let raw_input: Vec<String> = aoc_utils::input::load_input_lines(INPUT_FILE);
    let mut bots: Vec<Bot> = Vec::new();
    for line in raw_input {
        let re = Regex::new(r"-?\d+").unwrap();
        let matches: Vec<_> = re.find_iter(&line).map(|m| m.as_str().parse::<isize>().unwrap()).collect();
        bots.push(Bot::new(*matches.get(0).unwrap(), *matches.get(1).unwrap(), *matches.get(2).unwrap(), *matches.get(3).unwrap()));
    }
    let setup_time = start.elapsed();
    println!("Setup: {:.2?}", setup_time);
    
    start = Instant::now();
    solve_part_one(&mut bots.clone());
    let p1_time = start.elapsed();
    println!("Part 1: {:.2?}", setup_time + p1_time);
    
    start = Instant::now();
    solve_part_two(&mut bots.clone());
    let p2_time = start.elapsed();
    println!("Part 2: {:.2?}", setup_time + p2_time);
}

fn solve_part_one(bots: &mut Vec<Bot>) {
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    for bot in &mut *bots {
        bot.tick_n_times(100);
        match bot.get_quadrant() {
            Quadrant::ONE => {
                q1 += 1;
            },
            Quadrant::TWO => {
                q2 += 1;
            },
            Quadrant::THREE => {
                q3 += 1;
            },
            Quadrant::FOUR => {
                q4 += 1;
            },
            Quadrant::NONE => (),
        }
    }
    println!("Day 14 Part 1: {}", q1 * q2 * q3 * q4);
}

fn solve_part_two(bots: &mut Vec<Bot>) {
    let mut solution = 0;
    for n in 0..10000 {
        for bot in &mut *bots {
            bot.tick_n_times(1);
        }
        println!("ITERATION {}", n);
        print_bots(bots);
        println!("\n\n");
    }
    println!("Day 14 Part 2: {}", solution);
}

fn print_bots(bots: &mut Vec<Bot>) {
    let mut grid: [[char; X_MAX as usize]; Y_MAX as usize] = [[' '; X_MAX as usize]; Y_MAX as usize];
    for bot in bots {
        grid[bot.py as usize][bot.px as usize] = '#';
    }
    for y in 0..Y_MAX {
        let row: String = grid[y as usize].iter().collect();
        println!("{}", row);
    }
    println!("\n");
}

#[cfg(test)]
mod tests {
    use super::*;

}