use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let top_three = aoc::load_input("one")?
        .lines()
        .fold(vec![0], |mut res, l| {
            if l.is_empty() {
                res.push(0);
                res
            } else {
                let last = res.len() - 1;
                res[last] += l.parse::<u32>().unwrap();
                res
            }
        })
        .into_iter()
        .sorted_unstable()
        .rev()
        .take(3)
        .collect_vec();
    println!("Part one: {}", top_three.iter().max().unwrap());
    println!("Part two: {}", top_three.iter().sum::<u32>());
    Ok(())
}
