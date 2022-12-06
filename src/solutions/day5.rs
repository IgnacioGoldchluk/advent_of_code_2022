use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Move {
    pub quantity: u64,
    pub source: u64,
    pub destination: u64,
}

impl Move {
    pub fn new(move_str: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+) from (\d+) to (\d+)").unwrap();
        }

        let capture = RE.captures_iter(move_str).next().unwrap();
        Move {
            quantity: capture[1].parse::<u64>().unwrap(),
            source: capture[2].parse::<u64>().unwrap(),
            destination: capture[3].parse::<u64>().unwrap(),
        }
    }
}

pub fn solution() {
    let mut initial_state = vec![
        vec![],
        vec!['R', 'G', 'J', 'B', 'T', 'V', 'Z'],
        vec!['J', 'R', 'V', 'L'],
        vec!['S', 'Q', 'F'],
        vec!['Z', 'H', 'N', 'L', 'F', 'V', 'Q', 'G'],
        vec!['R', 'Q', 'T', 'J', 'C', 'S', 'M', 'W'],
        vec!['S', 'W', 'T', 'C', 'H', 'F'],
        vec!['D', 'Z', 'C', 'V', 'F', 'N', 'J'],
        vec!['L', 'G', 'Z', 'D', 'W', 'R', 'F', 'Q'],
        vec!['J', 'B', 'W', 'V', 'P'],
    ];
    fs::read_to_string("inputs/day5_input")
        .unwrap()
        .split('\n')
        .map(Move::new)
        .fold(&mut initial_state, apply_move2);
    print_answer(&initial_state);
}

fn print_answer(state: &[Vec<char>]) {
    let message: String = state.iter().skip(1).map(|v| v.last().unwrap()).collect();
    println!("{}", message);
}

fn apply_move(state: &mut Vec<Vec<char>>, move_cmd: Move) -> &mut Vec<Vec<char>> {
    for _ in 0..move_cmd.quantity {
        let elf_crate = state[move_cmd.source as usize].pop().unwrap();
        state[move_cmd.destination as usize].push(elf_crate);
    }
    state
}

fn apply_move2(state: &mut Vec<Vec<char>>, move_cmd: Move) -> &mut Vec<Vec<char>> {
    let mut chars = vec![];
    for _ in 0..move_cmd.quantity {
        chars.push(state[move_cmd.source as usize].pop().unwrap());
    }
    chars.reverse();

    state[move_cmd.destination as usize].append(&mut chars);
    state
}
