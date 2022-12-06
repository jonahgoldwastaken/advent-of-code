use std::{iter::Enumerate, slice::Windows};

use anyhow::{anyhow, Result};
use itertools::Itertools;

fn find_marker(mut windows: Enumerate<Windows<char>>) -> Result<usize> {
    let w = windows
        .find(|(_, w)| w.iter().unique().count() == w.len())
        .ok_or_else(|| {
            anyhow!(
                "couldn't find marker of size {} in stream",
                windows.next().unwrap().1.len()
            )
        })?;
    Ok(w.0 + w.1.len())
}

fn main() -> Result<()> {
    let input = aoc::load_input("six")?;
    let chars = input.trim().chars().collect_vec();
    println!("Part one: {}", find_marker(chars.windows(4).enumerate())?);
    println!("Part two: {}", find_marker(chars.windows(14).enumerate())?);
    Ok(())
}
