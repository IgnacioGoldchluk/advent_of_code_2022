use std::collections::HashSet;
use std::fs;

pub fn solution() {
    println!("Part1: {}", first_unique(4));
    println!("Part2: {}", first_unique(14));
}

fn first_unique(num: usize) -> usize {
    fs::read_to_string("inputs/day6_input")
        .unwrap()
        .chars()
        .collect::<Vec<char>>()
        .windows(num as usize)
        .map(|chunk| -> HashSet<_> { HashSet::from_iter(chunk) })
        .position(|set| set.len() == num)
        .unwrap()
        + num
}
