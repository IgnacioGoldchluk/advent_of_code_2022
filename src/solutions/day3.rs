use std::collections::HashSet;
use std::fs;

fn read_file() -> Vec<String> {
    fs::read_to_string("inputs/day3_input")
        .unwrap()
        .split("\n")
        .map(|string| string.to_string())
        .collect::<Vec<String>>()
}

pub fn solution() {
    let file_contents = read_file();

    let result: u64 = file_contents
        .iter()
        .map(|bag_contents| bag_contents.split_at(bag_contents.len() / 2))
        .map(overlapping_character)
        .map(priority)
        .sum();

    println!("Part1: {}", result);

    let result: u64 = file_contents
        .chunks(3)
        .map(overlapping_character_group)
        .map(priority)
        .sum();

    println!("Part2: {}", result);
}

fn overlapping_character_group(groups: &[String]) -> char {
    let mut result = groups
        .iter()
        .map(|x| HashSet::from_iter(x.chars()))
        .collect::<Vec<HashSet<_>>>();

    let (intersection, sets) = result.split_at_mut(1);

    let intersection = &mut intersection[0];
    for set in sets {
        intersection.retain(|e| set.contains(e));
    }
    intersection.iter().next().unwrap().clone()
}

fn priority(character: char) -> u64 {
    if character.is_uppercase() {
        character as u64 - 'A' as u64 + 27
    } else {
        character as u64 - 'a' as u64 + 1
    }
}

fn overlapping_character(halves: (&str, &str)) -> char {
    let first: HashSet<_> = HashSet::from_iter(halves.0.chars());
    let second: HashSet<_> = HashSet::from_iter(halves.1.chars());

    first.intersection(&second).next().unwrap().clone()
}
