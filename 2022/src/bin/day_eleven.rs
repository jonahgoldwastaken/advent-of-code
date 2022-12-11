use anyhow::Result;
use itertools::Itertools;
use std::{collections::VecDeque, default::Default};

#[derive(Default, Debug, Clone)]
enum Op {
    Add(u64),
    Mul(u64),
    Pow,
    #[default]
    Unset,
}

#[derive(Default, Debug, Clone)]
struct Monkey {
    items: VecDeque<u64>,
    op: Op,
    test: u64,
    test_true: usize,
    test_false: usize,
    inspected: usize,
}

fn main() -> Result<()> {
    let input = aoc::load_input("eleven")?;
    let monkeys = input
        .split("\n\n")
        .map(|s| {
            s.lines().skip(1).fold(Monkey::default(), |res, line| {
                let mut res = res;
                match line
                    .trim_start()
                    .chars()
                    .take_while(|c| *c != ':')
                    .collect::<String>()
                    .as_str()
                {
                    "Starting items" => {
                        res.items = line
                            .trim()
                            .replace("Starting items: ", "")
                            .split(", ")
                            .map(|s| s.trim().parse::<u64>().unwrap())
                            .collect()
                    }
                    "Operation" => {
                        let op_str = line.trim().replace("Operation: ", "");
                        let mut op_str = op_str.split(' ');
                        let last = op_str.clone().last().unwrap();
                        match op_str.nth(3).unwrap() {
                            "*" => {
                                res.op = if last == "old" {
                                    Op::Pow
                                } else {
                                    Op::Mul(last.parse::<u64>().unwrap())
                                };
                            }
                            "+" => res.op = Op::Add(last.parse::<u64>().unwrap()),
                            _ => (),
                        }
                    }
                    "Test" => {
                        res.test = line.split(' ').last().unwrap().parse::<u64>().unwrap();
                    }
                    x if x.starts_with("If ") => {
                        let monkey_idx = line.split(' ').last().unwrap().parse::<usize>().unwrap();
                        if x.ends_with("true") {
                            res.test_true = monkey_idx;
                        } else {
                            res.test_false = monkey_idx
                        }
                    }

                    _ => (),
                };
                res
            })
        })
        .collect_vec();
    println!("Part one: {}", calculate(monkeys.clone(), 20, true));
    println!("Part two: {}", calculate(monkeys, 10000, false));
    Ok(())
}

fn calculate(mut monkeys: Vec<Monkey>, rounds: usize, divide: bool) -> usize {
    let mo = monkeys.iter().fold(1, |res, m| res * m.test);
    for _ in 0..rounds {
        for m_idx in 0..monkeys.len() {
            let monkey = monkeys[m_idx].to_owned();
            monkey.items.into_iter().for_each(|level| {
                let mut worry_level = match monkey.op {
                    Op::Pow => (level % mo) * (level % mo) % mo,
                    Op::Add(i) => (level + i) % mo,
                    Op::Mul(i) => (level % mo) * (i % mo) % mo,
                    _ => 1,
                };
                if divide {
                    worry_level /= 3;
                }
                if (worry_level % monkey.test) == 0 {
                    monkeys[monkey.test_true].items.push_back(worry_level);
                } else {
                    monkeys[monkey.test_false].items.push_back(worry_level);
                }
            });

            let monkey = monkeys.get_mut(m_idx).unwrap();
            monkey.inspected += monkey.items.len();
            monkey.items = VecDeque::new();
        }
    }
    monkeys
        .iter()
        .map(|m| m.inspected)
        .sorted()
        .rev()
        .take(2)
        .product::<usize>()
}
