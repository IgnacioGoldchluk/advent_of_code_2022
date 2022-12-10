use std::fs;

use itertools::Itertools;

pub fn solution() {
    let operations = fs::read_to_string("inputs/day10_input").unwrap();

    let cycles = vec![20, 60, 100, 140, 180, 220];
    let mut register_values = vec![0]; // start with dummy value so we don't off by one later.

    let mut x = 1;
    for operation in operations.lines() {
        match operation.split_once(' ') {
            Some((_, value)) => {
                register_values.push(x);
                register_values.push(x);
                x += value.parse::<i32>().unwrap();
            }
            None => register_values.push(x),
        }
    }

    let result: i32 = register_values
        .iter()
        .enumerate()
        .filter(|(idx, _val)| cycles.iter().contains(idx))
        .map(|(idx, val)| idx as i32 * val)
        .sum();

    println!("{}", result);

    register_values
        .iter()
        .enumerate()
        .map(to_character)
        .chunks(40)
        .into_iter()
        .for_each(|line| println!("{}", line.into_iter().collect::<String>()));
}

fn to_character(idx_val: (usize, &i32)) -> char {
    let (idx, val) = idx_val;
    let idx = idx as i32;

    if (idx % 40) >= *val && (idx % 40) <= *val + 2 {
        '#'
    } else {
        '.'
    }
}
