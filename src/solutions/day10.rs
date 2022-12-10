use std::fs;

pub fn solution() {
    let cycles = vec![20, 60, 100, 140, 180, 220];

    let state_by_cycle = fs::read_to_string("inputs/day10_input")
        .unwrap()
        .lines()
        .flat_map(to_instructions)
        .map(|instruction| to_cycle_value(instruction, &mut state))
        .collect::<Vec<(i64, i64)>>();

    let result: i64 = state_by_cycle
        .iter()
        .filter(|(_x, cycle)| cycles.iter().any(|valid_cycles| valid_cycles == cycle))
        .map(|(x, cycle)| x * cycle)
        .sum();

    println!("{}", result);

    let screen = state_by_cycle
        .iter()
        .map(dot_or_numeral)
        .collect::<Vec<char>>();

    print_screen(&screen);
}

fn dot_or_numeral(state_cycle: &(i64, i64)) -> char {
    let (state, cycle) = *state_cycle;
    println!("{:?}", state_cycle);
    if state - 1 <= (cycle - 2 % 40) && (cycle % 40) <= state + 1 {
        '#'
    } else {
        '.'
    }
}

fn print_screen(screen: &Vec<char>) {
    screen
        .chunks(40)
        .map(|chunk| -> String { chunk.into_iter().collect() })
        .for_each(|chunk| println!("{}", chunk));
}

fn to_cycle_value(instruction: Instruction, state: &mut State) -> (i64, i64) {
    match instruction {
        Instruction::Noop => {}
        Instruction::Addx(value) => state.x += value,
    }
    state.cycle += 1;
    (state.x, state.cycle)
}

fn to_instructions(line: &str) -> Vec<Instruction> {
    match line {
        "noop" => vec![Instruction::Noop],
        _addx => vec![
            Instruction::Noop,
            Instruction::Addx(line.replace("addx ", "").parse::<i64>().unwrap()),
        ],
    }
}
