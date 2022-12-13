use anyhow::Result;
use itertools::Itertools;

fn char_to_points(c: char) -> u64 {
	if c > 'a' {
		c as u64 - 'a' as u64 + 1
	} else {
		c as u64 - 'A' as u64 + 27
	}
}

fn main() -> Result<()> {
	let sacks = aoc::load_input("three")?;
	let total: u64 = sacks
		.lines()
		.map(|l| l.trim().split_at(l.len() / 2))
		.map(|l| l.0.chars().find(|c| l.1.contains(*c)).unwrap())
		.map(char_to_points)
		.sum();
	println!("Part one: {total}");

	let group_total: u64 = sacks
		.lines()
		.tuples::<(&str, &str, &str)>()
		.map(|g| {
			g.0
				.chars()
				.find(|c| g.1.contains(*c) && g.2.contains(*c))
				.unwrap()
		})
		.map(char_to_points)
		.sum();
	println!("Part two: {group_total}");

	Ok(())
}
