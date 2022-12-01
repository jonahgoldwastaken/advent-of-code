use anyhow::Result;

fn main() -> Result<()> {
    let mut calories_per_elf: Vec<u32> =
        aoc::load_input("one")?.lines().fold(vec![0], |mut res, l| {
            if l.is_empty() {
                res.push(0);
                res
            } else {
                let last = res.len() - 1;
                res[last] += l.parse::<u32>().unwrap();
                res
            }
        });
    calories_per_elf.sort_unstable();
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
