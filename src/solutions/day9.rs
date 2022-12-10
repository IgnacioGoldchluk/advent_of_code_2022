use std::collections::HashSet;
use std::fs;

// TODO: Refactor to functional code
type Point = (i32, i32);

pub fn solution() {
    let commands: Vec<String> = fs::read_to_string("inputs/day9_input")
        .unwrap()
        .lines()
        .map(|line| line.to_owned())
        .collect();

    let mut rope = vec![(0, 0); 2];
    println!("{}", solve(&commands, &mut rope));
    let mut rope = vec![(0, 0); 10];
    println!("{}", solve(&commands, &mut rope));
}

fn solve(moves: &[String], rope: &mut [Point]) -> usize {
    let mut visited: HashSet<Point> = HashSet::new();
    for mov in moves {
        let (x, y, n) = match mov.split_at(2) {
            ("R ", n) => (1, 0, n),
            ("L ", n) => (-1, 0, n),
            ("U ", n) => (0, 1, n),
            ("D ", n) => (0, -1, n),
            (_, _) => unreachable!(),
        };

        for _ in 0..n.parse::<usize>().unwrap() {
            rope[0].0 += x;
            rope[0].1 += y;
            for i in 1..rope.len() {
                if let Some(pos) = move_adjacent(&rope[i], &rope[i - 1]) {
                    rope[i] = pos;
                } else {
                    break;
                }
            }
            visited.insert(*rope.last().unwrap());
        }
    }
    visited.len()
}

fn move_adjacent(tail: &Point, head: &Point) -> Option<Point> {
    let dx = tail.0 - head.0;
    let dy = tail.1 - head.1;

    if (dx == 2 || dx == -2) && (dy == 2 || dy == -2) {
        Some((head.0 + dx.clamp(-1, 1), head.1 + dy.clamp(-1, 1)))
    } else if dx == 2 || dx == -2 {
        Some((head.0 + dx.clamp(-1, 1), head.1))
    } else if dy == 2 || dy == -2 {
        Some((head.0, head.1 + dy.clamp(-1, 1)))
    } else {
        None // already adjacent
    }
}
