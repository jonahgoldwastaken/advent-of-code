use anyhow::{Error, Result};
use itertools::Itertools;

fn range_converter(st: &str) -> Result<(u8, u8)> {
    st.split('-')
        .map(|s| s.parse::<u8>())
        .fold_ok(
            (0, 0),
            |res, d| if res.0 > 0 { (res.0, d) } else { (d, res.1) },
        )
        .map_err(Error::from)
}

fn main() -> Result<()> {
    let input = aoc::load_input("four")?;
    let pairs: Vec<((u8, u8), (u8, u8))> = input
        .lines()
        .filter_map(|l| l.split(',').collect_tuple())
        .map(|(start, end)| {
            (
                range_converter(start).unwrap(),
                range_converter(end).unwrap(),
            )
        })
        .collect_vec();

    println!(
        "Part one: {}",
        pairs
            .iter()
            .filter(|((s1, e1), (s2, e2))| (s1 >= s2 && e1 <= e2) || (s2 >= s1 && e2 <= e1))
            .collect_vec()
            .len()
    );

    println!(
        "Part two: {}",
        pairs
            .iter()
            .filter(
                |((s1, e1), (s2, e2))| ((s1 <= s2 && e1 >= s2) || (s1 <= e2 && e1 >= e2))
                    || ((s2 <= s1 && e2 >= s1) || (s2 <= e1 && e2 >= e1))
            )
            .collect_vec()
            .len()
    );

    Ok(())
}
