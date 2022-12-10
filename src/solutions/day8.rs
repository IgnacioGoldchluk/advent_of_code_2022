use std::collections::HashMap;
use std::fs;

type Grid = HashMap<(i32, i32), i32>;

pub fn solution() {
    let grid = build_grid("inputs/day8_input".to_string());

    let visible: i32 = grid
        .iter()
        .map(|(coords, val)| is_visible(coords, *val, &grid) as i32)
        .sum();
    println!("{}", visible);

    let max_scenic_score = grid
        .iter()
        .map(|(coords, val)| scenic_score(coords, *val, &grid))
        .max()
        .unwrap();

    println!("{}", max_scenic_score);
}

fn is_visible(coords: &(i32, i32), val: i32, grid: &Grid) -> bool {
    let (x, y) = coords;
    visible(val, grid, (0..*y).map(|col| (*x, col)))
        || visible(val, grid, (*y + 1..).map(|col| (*x, col)))
        || visible(val, grid, (0..*x).map(|row| (row, *y)))
        || visible(val, grid, (*x + 1..).map(|row| (row, *y)))
}

fn visible<I>(val: i32, grid: &Grid, points: I) -> bool
where
    I: Iterator<Item = (i32, i32)>,
{
    points
        .take_while(|point| grid.contains_key(point))
        .map(|point| grid.get(&point).unwrap())
        .all(|x| *x < val)
}

fn scenic_score(coords: &(i32, i32), val: i32, grid: &Grid) -> i32 {
    let (x, y) = coords;
    score(val, grid, (0..*y).rev().map(|col| (*x, col)))
        * score(val, grid, (*y + 1..).map(|col| (*x, col)))
        * score(val, grid, (0..*x).rev().map(|row| (row, *y)))
        * score(val, grid, (*x + 1..).map(|row| (row, *y)))
}

fn score<I>(val: i32, grid: &Grid, points: I) -> i32
where
    I: Iterator<Item = (i32, i32)>,
{
    let result = points
        .into_iter()
        .try_fold(0, |acc, point| match grid.get(&point) {
            Some(number) => {
                if *number < val {
                    Ok(acc + 1)
                } else {
                    Err(acc + 1)
                }
            }
            None => Err(acc),
        });

    match result {
        Err(acc) => acc,
        Ok(acc) => acc,
    }
}

fn build_grid(filename: String) -> Grid {
    fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .enumerate()
        .fold(HashMap::new(), |mut grid, (row_idx, row)| {
            row.chars().enumerate().for_each(|(col_idx, value)| {
                grid.insert(
                    (row_idx as i32, col_idx as i32),
                    value.to_string().parse::<i32>().unwrap(),
                );
            });
            grid
        })
}
