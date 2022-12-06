use anyhow::{anyhow, Result};
use itertools::Itertools;

// Shape and round result points
const ROCK: u8 = 1;
const PAPER: u8 = 2;
const SCISSORS: u8 = 3;

const WIN: u8 = 6;
const DRAW: u8 = 3;
const LOSS: u8 = 0;

// All round scores based on the opponent's choice
const ROCK_WIN: u8 = PAPER + WIN;
const ROCK_DRAW: u8 = ROCK + DRAW;
const ROCK_LOSS: u8 = SCISSORS + LOSS;

const PAPER_WIN: u8 = SCISSORS + WIN;
const PAPER_DRAW: u8 = PAPER + DRAW;
const PAPER_LOSS: u8 = ROCK + LOSS;

const SCISSORS_WIN: u8 = ROCK + WIN;
const SCISSORS_DRAW: u8 = SCISSORS + DRAW;
const SCISSORS_LOSS: u8 = PAPER + LOSS;

fn calculate_round(them: char, you: Option<char>, result: Option<char>) -> Result<u8> {
    if let Some(you) = you {
        let round = them.to_string() + &you.to_string();
        Ok(match round.as_str() {
            "AX" => ROCK_DRAW,
            "AY" => ROCK_WIN,
            "AZ" => ROCK_LOSS,
            "BX" => PAPER_LOSS,
            "BY" => PAPER_DRAW,
            "BZ" => PAPER_WIN,
            "CX" => SCISSORS_WIN,
            "CY" => SCISSORS_LOSS,
            "CZ" => SCISSORS_DRAW,
            _ => unreachable!(),
        })
    } else if let Some(result) = result {
        let round = them.to_string() + &result.to_string();
        Ok(match round.as_str() {
            "AX" => ROCK_LOSS,
            "AY" => ROCK_DRAW,
            "AZ" => ROCK_WIN,
            "BX" => PAPER_LOSS,
            "BY" => PAPER_DRAW,
            "BZ" => PAPER_WIN,
            "CX" => SCISSORS_LOSS,
            "CY" => SCISSORS_DRAW,
            "CZ" => SCISSORS_WIN,
            _ => unreachable!(),
        })
    } else {
        Err(anyhow!(
            "supply either 'you' as Some(char) or 'result' as Some(char)"
        ))
    }
}

fn main() -> Result<()> {
    let input = aoc::load_input("two")?;

    let rounds: Vec<(char, char)> = input
        .lines()
        .map(|r| {
            r.trim()
                .split(' ')
                .map(|c| c.chars().next().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();

    println!(
        "Part one: {}",
        rounds
            .iter()
            .map(|(a, b)| calculate_round(*a, Some(*b), None).unwrap() as u32)
            .sum::<u32>()
    );
    println!(
        "Part two: {}",
        rounds
            .iter()
            .map(|(a, b)| calculate_round(*a, None, Some(*b)).unwrap() as u32)
            .sum::<u32>()
    );
    Ok(())
}
