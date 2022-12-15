use itertools::Itertools;
use serde_json;
use serde_json::Value;
use std::{cmp::Ordering, fs};

// TODO: Improve later

pub fn solution() {
    let packet_pairs: Vec<(Value, Value)> = fs::read_to_string("inputs/day13_input")
        .unwrap()
        .lines()
        .filter(|line| !line.is_empty())
        .map(to_json)
        .chunks(2)
        .into_iter()
        .map(|mut chunk| (chunk.next().unwrap(), chunk.next().unwrap()))
        .collect();

    let result: usize = packet_pairs
        .iter()
        .enumerate()
        .filter_map(|(idx, (c1, c2))| {
            if compare_packets(c1, c2).is_lt() {
                Some(idx + 1)
            } else {
                None
            }
        })
        .sum();
    println!("Part1: {result}");

    let divider_packets: Vec<Value> = vec!["[[2]]", "[[6]]"]
        .iter()
        .map(|x| serde_json::from_str(x).unwrap())
        .collect();

    let result: usize = packet_pairs
        .iter()
        .flat_map(|(c1, c2)| [c1, c2])
        .chain(&divider_packets)
        .cloned()
        .sorted_by(compare_packets)
        .positions(|packet| divider_packets.contains(&packet))
        .map(|idx| idx + 1)
        .product();

    println!("Part2: {result}");
}

fn to_json(line: &str) -> Value {
    serde_json::from_str(line).unwrap()
}

fn compare_packets(first: &Value, second: &Value) -> Ordering {
    match (first, second) {
        (Value::Number(x), Value::Number(y)) => x.as_u64().cmp(&y.as_u64()),
        (Value::Array(x), Value::Array(y)) => compare_arrays(x, y),
        (Value::Array(_), Value::Number(_)) => {
            compare_packets(first, &Value::Array(vec![second.clone()]))
        }
        (Value::Number(_), Value::Array(_)) => {
            compare_packets(&Value::Array(vec![first.clone()]), second)
        }
        _ => unreachable!(),
    }
}

fn compare_arrays(first: &[Value], second: &[Value]) -> Ordering {
    let mut first_iter = first.iter();
    let mut second_iter = second.iter();

    loop {
        match (first_iter.next(), second_iter.next()) {
            (None, None) => return Ordering::Equal,
            (None, _) => return Ordering::Less,
            (_, None) => return Ordering::Greater,
            (Some(x), Some(y)) => match compare_packets(x, y) {
                Ordering::Equal => {}
                ordering => return ordering,
            },
        }
    }
}
