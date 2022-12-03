use std::fs;

pub fn solution() {
    let result: u64 = fs::read_to_string("inputs/day2_input")
        .unwrap()
        .split("\n")
        .map(scores_part1)
        .sum();

    println!("Total part1: {}", result);

    let result: u64 = fs::read_to_string("inputs/day2_input")
        .unwrap()
        .split("\n")
        .map(scores_part2)
        .sum();

    println!("Total part2: {}", result);
}

fn scores_part1(round: &str) -> u64 {
    match round {
        "A X" => 4,
        "A Y" => 8,
        "A Z" => 3,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 7,
        "C Y" => 2,
        "C Z" => 6,
        _ => panic!(),
    }
}

fn scores_part2(round: &str) -> u64 {
    match round {
        "A X" => 3,
        "A Y" => 4,
        "A Z" => 8,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 2,
        "C Y" => 6,
        "C Z" => 7,
        _ => panic!(),
    }
}
