use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;
use rustc_hash::FxHashSet;
use std::fs;
use std::ops::Range;

type Point = (i64, i64);

pub fn solution() {
    let sensor_beacon = fs::read_to_string("inputs/day15_input")
        .unwrap()
        .lines()
        .map(parse_line)
        .collect::<Vec<(Point, Point)>>();

    let cleared = extreme_points(&sensor_beacon)
        .map(|x| (x, 2000000))
        .filter(|x| !sensor_beacon.iter().any(|(s, b)| s == x || b == x))
        .filter(|x| is_cleared(x, &sensor_beacon))
        .count();
    println!("Part1: {cleared}");

    let freq = find_unique_parallel(&sensor_beacon, 4_000_000);

    println!("Part2: {freq}");
}

fn find_unique_parallel(sensor_beacon: &[(Point, Point)], max: i64) -> i64 {
    let mut set = FxHashSet::default();
    for (s, b) in sensor_beacon {
        set.insert(s);
        set.insert(b);
    }
    let (x, y) = (0..=max)
        .cartesian_product(0..=max)
        .par_bridge()
        .filter(|x| !set.contains(x))
        .inspect(|(x, y)| print_if_divisible(*x, *y))
        .find_first(|p| !set.contains(p) && !is_cleared(p, &sensor_beacon))
        .unwrap();
    (x * 4_000_000) + y
}

fn print_if_divisible(x: i64, y: i64) {
    if x % 10_000 == 0 && y % 10_000 == 0 {
        println!("{x}, {y}");
    } else {
    }
}

fn extreme_points(sensor_beacon: &[(Point, Point)]) -> Range<i64> {
    let max_manhattan = sensor_beacon
        .iter()
        .map(|(s, b)| manhattan_distance(s, b))
        .max()
        .unwrap();

    let min_x = sensor_beacon.iter().map(|(s, _b)| s.0).min().unwrap();
    let max_x = sensor_beacon.iter().map(|(s, _b)| s.0).max().unwrap();
    min_x - max_manhattan..max_x + max_manhattan
}

fn is_cleared(point: &Point, sensor_beacon: &[(Point, Point)]) -> bool {
    sensor_beacon
        .iter()
        .any(|(s, b)| manhattan_distance(s, b) >= manhattan_distance(s, point))
}

fn parse_line(line: &str) -> (Point, Point) {
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap()
            .captures(line)
            .unwrap();
    (
        (p_i64(&re[1]), p_i64(&re[2])),
        (p_i64(&re[3]), p_i64(&re[4])),
    )
}

fn p_i64(x: &str) -> i64 {
    x.parse::<i64>().unwrap()
}

fn manhattan_distance(p1: &Point, p2: &Point) -> i64 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}
