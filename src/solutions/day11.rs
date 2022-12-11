use itertools::Itertools;
use regex::Regex;
use std::boxed::Box;
use std::collections::HashMap;
use std::fs;

struct Monkey {
    pub items: Vec<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    test: u64,
    throw: HashMap<bool, usize>,
    inspected: u64,
    pub gcd: u64,
}

impl Monkey {
    pub fn new(monkey_info: &str) -> Self {
        let items = Monkey::parse_items(monkey_info);
        let operation = Monkey::parse_operation(monkey_info);
        let test = Monkey::parse_test(monkey_info);
        let throw = Monkey::parse_throw(monkey_info);

        Monkey {
            items,
            operation,
            test,
            throw,
            inspected: 0,
            gcd: test,
        }
    }

    fn parse_throw(monkey_info: &str) -> HashMap<bool, usize> {
        let mut throw = HashMap::new();
        monkey_info
            .lines()
            .map(|line| line.trim())
            .filter(|line| line.starts_with("If"))
            .for_each(|line| {
                let re = Regex::new(r"If (true|false): throw to monkey (\d+)").unwrap();
                let captures = re.captures(line).unwrap();

                match (&captures[1], &captures[2]) {
                    ("true", to_monkey) => throw.insert(true, to_monkey.parse::<usize>().unwrap()),
                    ("false", to_monkey) => {
                        throw.insert(false, to_monkey.parse::<usize>().unwrap())
                    }
                    _ => unreachable!(),
                };
            });
        throw
    }

    fn parse_items(monkey_info: &str) -> Vec<u64> {
        let relevant_line = monkey_info
            .lines()
            .filter(|line| line.trim().starts_with("Starting items"))
            .map(|line| line.replace("  Starting items: ", ""))
            .next()
            .unwrap();

        relevant_line
            .split(", ")
            .map(|num| num.parse::<u64>().unwrap())
            .collect()
    }

    fn parse_test(monkey_info: &str) -> u64 {
        monkey_info
            .lines()
            .filter(|line| line.contains("Test:"))
            .map(|line| {
                line.trim()
                    .replace("Test: divisible by ", "")
                    .parse::<u64>()
                    .unwrap()
            })
            .next()
            .unwrap()
    }

    fn parse_operation(monkey_info: &str) -> Box<dyn Fn(u64) -> u64> {
        let captures = Regex::new(r"Operation: new = old (.) (\d+|old)")
            .unwrap()
            .captures(monkey_info)
            .unwrap();

        match (&captures[1], &captures[2]) {
            ("+", "old") => Box::new(move |number| number + number),
            ("*", "old") => Box::new(move |number| number * number),
            ("+", val) => {
                let val = val.parse::<u64>().unwrap();
                Box::new(move |number| number + val)
            }
            ("*", val) => {
                let val = val.parse::<u64>().unwrap();
                Box::new(move |number| number * val)
            }
            _ => unreachable!(),
        }
    }

    pub fn perform_round(&mut self, divisor: u64) -> Vec<(usize, u64)> {
        let mut operations: Vec<(usize, u64)> = Vec::new();
        self.items.iter().for_each(|item| {
            let worry_level = (self.operation)(*item) / divisor;
            let test = worry_level % self.test == 0;
            let to_throw = self.throw.get(&test).unwrap();
            operations.push((*to_throw, worry_level % self.gcd));
            self.inspected += 1;
        });
        self.items.clear();
        operations
    }
}

fn assign_gcd(monkeys: &mut Vec<Monkey>) {
    let gcd: u64 = monkeys.iter().map(|monkey| monkey.gcd).product();
    for monkey in monkeys.iter_mut() {
        monkey.gcd = gcd;
    }
}

pub fn solution() {
    let mut monkeys: Vec<Monkey> = fs::read_to_string("inputs/day11_input")
        .unwrap()
        .split("\n\n")
        .map(Monkey::new)
        .collect();

    assign_gcd(&mut monkeys);

    for _round in 0..20 {
        for idx in 0..monkeys.len() {
            let monkey = &mut monkeys[idx];
            let pushes = monkey.perform_round(3);
            for (target, val) in pushes.iter() {
                let other_monkey = &mut monkeys[*target];
                other_monkey.items.push(*val);
            }
        }
    }

    let result: u64 = monkeys
        .iter()
        .map(|monkey| monkey.inspected)
        .sorted()
        .rev()
        .take(2)
        .product();

    println!("{}", result)
}
