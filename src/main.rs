mod days;

use days::*;
use clap::Parser;

#[derive(Parser)]
struct Opts {
    day: u8,
}

fn main() {
    let cli = Opts::parse();

    let input_file = format!("inputs/{}.txt", cli.day);
    println!("Reading input from {}", input_file);
    let input = std::fs::read_to_string(input_file).expect("Failed to read input file");

    match cli.day {
        1 => day1::solve(&input),
        2 => day2::solve(&input),
        3 => day3::solve(&input),
        4 => day4::solve(&input),
        5 => day5::solve(&input),
        6 => day6::solve(&input),
        7 => day7::solve(&input),
        8 => day8::solve(&input),
        9 => day9::solve(&input),
        10 => day10::solve(&input),
        11 => day11::solve(&input),
        12 => day12::solve(&input),
        13 => day13::solve(&input),
        14 => day14::solve(&input),
        15 => day15::solve(&input),
        16 => day16::solve(&input),
        17 => day17::solve(&input),
        18 => day18::solve(&input),
        19 => day19::solve(&input),
        20 => day20::solve(&input),
        21 => day21::solve(&input),
        22 => day22::solve(&input),
        23 => day23::solve(&input),
        24 => day24::solve(&input),
        25 => day25::solve(&input),
        _ => println!("Day {} not implemented yet", cli.day),
    }

}
