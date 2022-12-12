#![allow(dead_code)]
use clap::Parser;

mod solutions;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Advent of Code day to Run.
    #[arg(long)]
    day: u8,
}

fn main() {
    let args = Args::parse();

    match args.day {
        1 => solutions::day1::solution(),
        2 => solutions::day2::solution(),
        3 => solutions::day3::solution(),
        4 => solutions::day4::solution(),
        5 => solutions::day5::solution(),
        6 => solutions::day6::solution(),
        7 => solutions::day7::solution(),
        8 => solutions::day8::solution(),
        9 => solutions::day9::solution(),
        10 => solutions::day10::solution(),
        11 => solutions::day11::solution(),
        12 => solutions::day12::solution(),
        _ => unreachable!(),
    }
}
