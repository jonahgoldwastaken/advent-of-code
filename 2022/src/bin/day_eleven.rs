use anyhow::Result;
use itertools::Itertools;
use std::{collections::VecDeque, default::Default};

#[derive(Default, Debug, Clone)]
enum WorryOperation {
    Addition(u64),
    Multiplication(u64),
    Double,
    #[default]
    Unset,
}

#[derive(Default, Debug, Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operation: WorryOperation,
    test: u64,
    test_true: usize,
    test_false: usize,
    items_inspected: usize,
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
                    "Starting items" => line
                        .trim()
                        .replace("Starting items: ", "")
                        .split(", ")
                        .map(|s| s.trim().parse::<u64>().unwrap())
                        .for_each(|item| res.items.push_back(item)),
                    "Operation" => {
                        line.trim()
                            .replace("Operation: ", "")
                            .split(' ')
                            .for_each(|s| match s {
                                "*" => res.operation = WorryOperation::Multiplication(0),
                                "+" => res.operation = WorryOperation::Addition(0),
                                x => {
                                    if let Ok(x) = x.parse::<u64>() {
                                        match res.operation {
                                            WorryOperation::Addition(_) => {
                                                res.operation = WorryOperation::Addition(x)
                                            }
                                            WorryOperation::Multiplication(_) => {
                                                res.operation = WorryOperation::Multiplication(x)
                                            }
                                            WorryOperation::Unset => {
                                                panic!("WorryOperation wasn't set properly")
                                            }
                                            _ => (),
                                        }
                                    } else if x == "old" {
                                        if let WorryOperation::Multiplication(_) = res.operation {
                                            res.operation = WorryOperation::Double
                                        }
                                    }
                                }
                            })
                    }
                    "Test" => {
                        res.test = line.split(' ').last().unwrap().parse::<u64>().unwrap();
                    }
                    "If true" => {
                        let monkey_idx = line.split(' ').last().unwrap().parse::<usize>().unwrap();
                        res.test_true = monkey_idx;
                    }
                    "If false" => {
                        let monkey_idx = line.split(' ').last().unwrap().parse::<usize>().unwrap();
                        res.test_false = monkey_idx
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
                let mut worry_level = match monkey.operation {
                    WorryOperation::Double => (level % mo) * (level % mo) % mo,
                    WorryOperation::Addition(i) => (level + i) % mo,
                    WorryOperation::Multiplication(i) => (level % mo) * (i % mo) % mo,
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
            monkey.items_inspected += monkey.items.len();
            monkey.items = VecDeque::new();
        }
    }
    monkeys
        .iter()
        .map(|m| m.items_inspected)
        .sorted()
        .rev()
        .take(2)
        .product::<usize>()
}
