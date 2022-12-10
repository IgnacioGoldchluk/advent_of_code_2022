use std::collections::HashSet;
use std::fs;

type Point = (i32, i32);

pub fn solution() {
    let commands: Vec<String> = fs::read_to_string("inputs/day9_input")
        .unwrap()
        .lines()
        .map(|line| line.to_owned())
        .collect();

    println!("{}", solve(&commands, 2));
    println!("{}", solve(&commands, 10));
}

fn solve(moves: &[String], length: usize) -> usize {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut rope = vec![(0, 0); length];

    moves
        .iter()
        .flat_map(to_move)
        .for_each(|movement| apply_movement(movement, &mut rope, &mut visited));

    visited.len()
}

fn to_move(line: &String) -> Vec<(i32, i32)> {
    let (dir, repeats) = match line.split_once(' ').unwrap() {
        ("R", n) => ((1, 0), n),
        ("L", n) => ((-1, 0), n),
        ("U", n) => ((0, 1), n),
        ("D", n) => ((0, -1), n),
        (_, _) => unreachable!(),
    };
    vec![dir; repeats.parse::<usize>().unwrap()]
}

fn apply_movement(
    movement: (i32, i32),
    rope: &mut Vec<(i32, i32)>,
    seen: &mut HashSet<(i32, i32)>,
) {
    rope[0].0 += movement.0;
    rope[0].1 += movement.1;

    for i in 1..rope.len() {
        if let Some(new_pos) = move_adjacent(&rope[i], &rope[i - 1]) {
            rope[i] = new_pos;
        }
    }
    seen.insert(*rope.last().unwrap());
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
        None
    }
}
