use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::{cmp::Ordering, hash::Hash};

#[derive(Debug, Eq, PartialOrd, Ord, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Hash for Coord {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

fn snaker(input: &str, knots_amount: usize) -> Result<usize> {
    let mut knots = vec![Coord { x: 0, y: 0 }; knots_amount];
    let directions: Vec<Vec<char>> = input
        .lines()
        .map(|l| {
            Ok(vec![
                l.chars().next().ok_or_else(|| anyhow!(
                    "couldn't get char from line"
                ))?;
                l.split(' ')
                    .last()
                    .ok_or_else(|| anyhow!("couldn't get last part of string"))?
                    .parse::<usize>()?
            ])
        })
        .collect::<Result<_>>()?;
    let directions = directions.concat();
    let coords: Vec<Coord> = directions
        .iter()
        .map(|dir| {
            let head = knots
                .get_mut(0)
                .ok_or_else(|| anyhow!("couldn't get head of knots"))?;
            match dir {
                'U' => {
                    head.y -= 1;
                }
                'D' => {
                    head.y += 1;
                }
                'L' => {
                    head.x -= 1;
                }
                'R' => {
                    head.x += 1;
                }
                _ => unreachable!("no other directions possible"),
            }
            for i in 1..knots.len() {
                let k1 = knots[i - 1].clone();
                let k2 = knots
                    .get_mut(i)
                    .ok_or_else(|| anyhow!("couldn't get knot at index {i}"))?;
                if !(k1.y - 1..=k1.y + 1).contains(&k2.y) || !(k1.x - 1..=k1.x + 1).contains(&k2.x)
                {
                    k2.x = match k1.x.cmp(&k2.x) {
                        Ordering::Less => k2.x - 1,
                        Ordering::Equal => k2.x,
                        Ordering::Greater => k2.x + 1,
                    };
                    k2.y = match k1.y.cmp(&k2.y) {
                        Ordering::Less => k2.y - 1,
                        Ordering::Equal => k2.y,
                        Ordering::Greater => k2.y + 1,
                    };
                }
            }
            Ok(knots
                .last()
                .ok_or_else(|| anyhow!("couldn't get last knot"))?
                .clone())
        })
        .collect::<Result<_>>()?;
    Ok(coords.iter().unique().count())
}

fn main() -> Result<()> {
    let input = aoc::load_input("nine")?;
    println!("Part one: {}", snaker(&input, 2)?);
    println!("Part two: {}", snaker(&input, 10)?);
    Ok(())
}
