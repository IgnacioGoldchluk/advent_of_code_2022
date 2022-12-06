use std::fs;

struct CleaningRange {
    pub start: u64,
    pub finish: u64,
}

impl CleaningRange {
    pub fn new(ranges: &str) -> Self {
        let (start, finish) = ranges.split_once('-').unwrap();
        CleaningRange {
            start: start.parse::<u64>().unwrap(),
            finish: finish.parse::<u64>().unwrap(),
        }
    }

    pub fn fully_contains(&self, other: &CleaningRange) -> bool {
        other.start >= self.start && other.finish <= self.finish
    }

    pub fn overlaps(&self, other: &CleaningRange) -> bool {
        (other.start <= self.finish && other.start >= self.finish)
            || (other.finish >= self.start && other.finish <= self.finish)
    }
}

fn solve<F>(func: F) -> u64
where
    F: Fn(&str) -> bool,
{
    fs::read_to_string("inputs/day4_input")
        .unwrap()
        .split('\n')
        .map(|ranges| func(ranges) as u64)
        .sum()
}

pub fn solution() {
    println!("Part1: {}", solve(is_fully_contained));
    println!("Part2: {}", solve(is_overlap));
}

fn is_overlap(ranges: &str) -> bool {
    let (first_range, second_range) = to_cleaning_ranges(ranges);
    first_range.overlaps(&second_range) || second_range.overlaps(&first_range)
}

fn to_cleaning_ranges(ranges: &str) -> (CleaningRange, CleaningRange) {
    let (first, second) = ranges.split_once(',').unwrap();
    (CleaningRange::new(first), CleaningRange::new(second))
}

fn is_fully_contained(ranges: &str) -> bool {
    let (first_range, second_range) = to_cleaning_ranges(ranges);
    first_range.fully_contains(&second_range) || second_range.fully_contains(&first_range)
}
