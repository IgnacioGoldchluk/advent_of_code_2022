use std::fs;

pub fn solution() {
    let mut calories = fs::read_to_string("inputs/day1_input")
        .unwrap()
        .split("\n\n")
        .map(to_calories)
        .collect::<Vec<u64>>();

    calories.sort();
    calories.reverse();

    println!("Solution 1: {:?}", &calories[0]);
    println!(
        "Solution 2: {:?}",
        &calories[0..3].into_iter().sum() as &u64
    );
}

fn to_calories(calories: &str) -> u64 {
    calories
        .split("\n")
        .map(|val| val.parse::<u64>().unwrap())
        .sum()
}
