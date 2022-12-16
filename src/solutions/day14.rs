use std::collections::HashMap;
use std::fs;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Element {
    Sand,
    Rock,
    Air,
}
type Point = (i32, i32);
type Grid = HashMap<Point, Element>;

fn get_grid(filename: &str) -> Grid {
    let mut grid: Grid = HashMap::new();

    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(to_points)
        .flatten()
        .for_each(|point| {
            grid.insert(point, Element::Rock);
        });
    grid
}

pub fn solution() {
    let mut grid = get_grid("inputs/day14_input");

    fill_with_sand(&mut grid);
    let sand = grid.values().filter(|e| **e == Element::Sand).count();
    println!("Part1: {sand}");

    let mut grid = get_grid("inputs/day14_input");
    fill_floor(&mut grid);
    fill_with_sand(&mut grid);

    let sand = grid.values().filter(|e| **e == Element::Sand).count();
    println!("Part2: {sand}");
}

fn fill_floor(grid: &mut Grid) {
    let max_y = *grid.keys().map(|(_x, y)| y).max().unwrap() + 2;
    for x in -max_y..=max_y {
        grid.insert((500 + x, max_y), Element::Rock);
    }
}

fn fill_with_sand(grid: &mut Grid) {
    let highest = *grid.keys().map(|(_x, y)| y).max().unwrap();
    while drop(grid, highest) && !grid.contains_key(&(500, 0)) {}
}

fn drop(grid: &mut Grid, highest: i32) -> bool {
    let mut start = (500, 0);

    while start.1 <= highest {
        let new = move_one(&grid, &start);
        if start == new {
            break;
        } else {
            start = new;
        }
    }

    if start.1 <= highest {
        grid.insert(start, Element::Sand);
    } else {
    }

    start.1 <= highest
}

fn print_grid(grid: &Grid) {
    for y in 0..=12 {
        println!();
        for x in 480..=520 {
            let c = match grid.get(&(x, y)).unwrap_or(&Element::Air) {
                Element::Rock => "#",
                Element::Air => ".",
                Element::Sand => "o",
            };
            print!("{c}");
        }
    }
    println!();
}

fn move_one(grid: &Grid, point: &(i32, i32)) -> (i32, i32) {
    let below = (point.0, point.1 + 1);
    let left_down = (point.0 - 1, point.1 + 1);
    let right_down = (point.0 + 1, point.1 + 1);

    vec![below, left_down, right_down]
        .iter()
        .filter(|x| *grid.get(&x).unwrap_or(&Element::Air) == Element::Air)
        .next()
        .unwrap_or(point)
        .clone()
}

fn to_points(line: &str) -> Vec<Point> {
    let mut points: Vec<Point> = vec![];
    let coordinates = line.split(" -> ").collect::<Vec<&str>>();

    for i in 1..coordinates.len() {
        let curr = coordinates[i];
        let prev = coordinates[i - 1];
        points.append(&mut chain_coordinates(curr, prev));
    }
    points
}

fn chain_coordinates(c1: &str, c2: &str) -> Vec<Point> {
    match (to_coordinate(c1), to_coordinate(c2)) {
        ((x1, y1), (x2, y2)) if x1 == x2 => {
            let i = if y1 > y2 { y2..=y1 } else { y1..=y2 };
            i.map(|y| (x1, y)).collect::<Vec<Point>>()
        }
        ((x1, y1), (x2, y2)) if y1 == y2 => {
            let i = if x1 > x2 { x2..=x1 } else { x1..=x2 };
            i.map(|x| (x, y1)).collect::<Vec<Point>>()
        }
        (_, _) => unreachable!(),
    }
}

fn to_coordinate(coord: &str) -> Point {
    let (x, y) = coord.split_once(',').unwrap();
    (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
}
