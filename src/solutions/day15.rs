use regex::Regex;
use std::boxed::Box;
use std::collections::HashSet;
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

    let freq = find_unique(&sensor_beacon, (0, 4_000_000));

    println!("Part2: {freq}");
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

fn find_unique(sensor_beacon: &[(Point, Point)], limits: (i64, i64)) -> i64 {
    let mut set = HashSet::new();
    for (s, b) in sensor_beacon {
        set.insert(s);
        set.insert(b);
    }

    let (x, y) = sensor_beacon
        .iter()
        .flat_map(external_borders)
        .filter(|point| !set.contains(point) && within_limits(point, &limits))
        .find(|point| !is_cleared(point, &sensor_beacon))
        .unwrap();
    (x * 4_000_000) + y
}

fn within_limits(point: &Point, limits: &(i64, i64)) -> bool {
    point.0 <= limits.1 && point.0 >= limits.0 && point.1 <= limits.1 && point.1 >= limits.0
}

fn external_borders((sensor, beacon): &(Point, Point)) -> Box<dyn Iterator<Item = Point> + '_> {
    let distance = manhattan_distance(sensor, beacon) + 1;
    let slope_1 = (0..=distance).map(move |x| (sensor.0 - x, sensor.1 - (distance - x)));
    let slope_2 = (0..=distance).map(move |x| (sensor.0 - x, sensor.1 + (distance - x)));
    let slope_3 = (0..=distance).map(move |x| (sensor.0 + x, sensor.1 + (distance - x)));
    let slope_4 = (0..=distance).map(move |x| (sensor.0 + x, sensor.1 - (distance - x)));

    Box::new(slope_1.chain(slope_2).chain(slope_3).chain(slope_4))
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
