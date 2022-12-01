use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let mut calories_per_elf: Vec<u32> = aoc::load_input("one")?
        .lines()
        .fold(vec![vec![]], |mut res, l| {
            if l.is_empty() {
                res.push(Vec::new());
                res
            } else {
                res.last_mut().unwrap().push(l.parse::<u32>().unwrap());
                res
            }
        })
        .iter()
        .map(|e| e.iter().sum::<u32>())
        .sorted()
        .collect_vec();
    println!("Part one: {}", calories_per_elf.iter().max().unwrap());
    println!(
        "Part two: {}",
        calories_per_elf
            .split_off(calories_per_elf.len() - 3)
            .into_iter()
            .sum::<u32>()
    );
    Ok(())
}
