use anyhow::Result;
use itertools::Itertools;
use std::collections::VecDeque;

fn get_board(input: &str) -> Vec<VecDeque<char>> {
    let mut raw_board = input
        .lines()
        .take_while(|l| !l.trim().is_empty())
        .collect_vec();
    let indices = raw_board
        .pop()
        .unwrap()
        .split(' ')
        .filter_map(|s| s.parse::<usize>().ok())
        .map(|n| n - 1)
        .collect_vec();
    indices
        .iter()
        .map(|i| {
            let mut vec: VecDeque<char> = VecDeque::new();
            for line in raw_board.iter().rev() {
                let char = line.chars().nth(4 * i + 1).unwrap();
                if char != ' ' {
                    vec.push_back(char)
                }
            }
            vec
        })
        .collect_vec()
}

fn get_moves(input: &str) -> Vec<(usize, usize, usize)> {
    let lines = input
        .lines()
        .skip_while(|l| !l.trim().is_empty())
        .skip(1)
        .collect_vec();
    lines
        .iter()
        .map(|l| {
            l.split(' ')
                .filter_map(|s| s.parse::<usize>().ok())
                .collect_tuple()
                .unwrap()
        })
        .map(|(amnt, from, to)| (amnt, from - 1, to - 1))
        .collect_vec()
}

fn get_top(board: Vec<VecDeque<char>>) -> String {
    board
        .iter()
        .map(|stack| *stack.back().unwrap())
        .collect::<String>()
}

fn main() -> Result<()> {
    let input = aoc::load_input("five")?;
    let board = get_board(&input);
    let moves = get_moves(&input);

    let mut board_one = board.clone();
    for (amount, from, to) in moves.iter() {
        let mut amnt = *amount;
        let mut crates = Vec::with_capacity(amnt);

        while amnt > 0 {
            crates.push(board_one.get_mut(*from).unwrap().pop_back().unwrap());
            amnt -= 1;
        }

        board_one.get_mut(*to).unwrap().extend(crates);
    }

    println!("Part one: {}", get_top(board_one));

    let mut board_two = board.clone();
    for (amount, from, to) in moves {
        let mut amnt = amount;
        let mut crates = Vec::with_capacity(amnt);
        while amnt > 0 {
            crates.push(board_two.get_mut(from).unwrap().pop_back().unwrap());
            amnt -= 1;
        }
        crates.reverse();

        board_two.get_mut(to).unwrap().extend(crates);
    }

    println!("Part two: {}", get_top(board_two));

    Ok(())
}
