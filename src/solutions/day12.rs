use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

type Grid = HashMap<(i64, i64), char>;

pub fn solution() {
    let grid = read_grid("inputs/day12_input");

    let start = find_key(&grid, 'S').unwrap();
    let target = find_key(&grid, 'E').unwrap();
    println!("Part1: {}", a_star_algo(&start, &target, &grid));

    let result = grid
        .iter()
        .filter(|(_k, v)| **v == 'a')
        .map(|(pos, _char)| a_star_algo(pos, &target, &grid))
        .min()
        .unwrap();
    println!("Part2: {}", result);
}
fn find_key(grid: &Grid, target: char) -> Option<(i64, i64)> {
    grid.iter()
        .find_map(|(key, value)| if *value == target { Some(*key) } else { None })
}

fn read_grid(filename: &str) -> Grid {
    let mut grid = HashMap::new();
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .enumerate()
        .for_each(|(row, line)| {
            line.chars().enumerate().for_each(|(col, character)| {
                grid.insert((row as i64, col as i64), character);
            })
        });
    grid
}
fn get_neighbors(point: &(i64, i64), grid: &Grid) -> Vec<(i64, i64)> {
    let directions = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    directions
        .iter()
        .map(|dir| (point.0 + dir.0, point.1 + dir.1))
        .filter(|neighbor| {
            grid.contains_key(neighbor)
                && is_visitable(grid.get(point).unwrap(), grid.get(neighbor).unwrap())
        })
        .collect()
}
fn is_visitable(current: &char, neighbor: &char) -> bool {
    height(neighbor) <= height(current) + 1
}
fn height(point: &char) -> u32 {
    match point {
        'S' => 0,
        'E' => (b'z' as u32) - (b'a' as u32),
        x => (*x as u32) - (b'a' as u32),
    }
}
fn a_star_algo(start: &(i64, i64), target: &(i64, i64), grid: &Grid) -> u64 {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    queue.push_back((*start, 0));
    seen.insert(*start);
    while let Some((point, distance)) = queue.pop_front() {
        if point == *target {
            return distance;
        }
        for neighbor in get_neighbors(&point, grid) {
            if !seen.contains(&neighbor) {
                seen.insert(neighbor);
                queue.push_back((neighbor, distance + 1));
            }
        }
    }
    std::u64::MAX
}
