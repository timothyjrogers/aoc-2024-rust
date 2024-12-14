use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut day: i32 = -1;
    if args.len() > 1 {
        day = args[1].parse::<i32>().unwrap();
    }
    if day == -1 {
        day1::solve();
        day2::solve();
        day3::solve();
        day4::solve();
        day5::solve();
        day6::solve();
        day7::solve();
        day8::solve();
        day9::solve();
        day10::solve();
        day11::solve();
        day12::solve();
        //day13::solve();
        //day14::solve();
        //day15::solve();
        //day16::solve();
        //day17::solve();
        //day18::solve();
        //day19::solve();
        //day20::solve();
        //day21::solve();
        //day22::solve();
        //day23::solve();
        //day24::solve();
        //day25::solve();
    } else if day == 1 {
        day1::solve();
    } else if day == 2 {
        day2::solve();
    } else if day == 3 {
        day3::solve();
    } else if day == 4 {
        day4::solve();
    } else if day == 5 {
        day5::solve();
    } else if day == 6 {
        day6::solve();
    } else if day == 7 {
        day7::solve();
    } else if day == 8 {
        day8::solve();
    } else if day == 9 {
        day9::solve();
    } else if day == 10 {
        day10::solve();
    } else if day == 11 {
        day11::solve();
    } else if day == 12 {
        day12::solve();
    }
}