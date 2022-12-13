use anyhow::Result;
use aoc::Grid;
use itertools::Itertools;

fn main() -> Result<()> {
    let grid: Grid<u8> = Grid::from(
        aoc::load_input("eight")?
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| String::from(c).parse().unwrap())
                    .collect_vec()
            })
            .collect_vec(),
    );

    println!("Part one: {}", part_one(&grid)?);
    println!("Part two: {}", part_two(&grid)?);
    Ok(())
}

fn part_one(grid: &Grid<u8>) -> Result<usize> {
    match grid
        .rows()
        .iter()
        .enumerate()
        .map(|(i, row)| {
            match row
                .iter()
                .enumerate()
                .map(|(j, tree)| {
                    let col = grid.col(j)?;
                    Ok((visible_left(row, j, tree)
                        || visible_right(row, j, tree)
                        || visible_left(col, i, tree)
                        || visible_right(col, i, tree)) as usize)
                })
                .collect::<Result<Vec<usize>>>()
            {
                Ok(v) => Ok(v.into_iter().sum()),
                Err(e) => Err(e),
            }
        })
        .collect::<Result<Vec<usize>>>()
    {
        Ok(v) => Ok(v.into_iter().sum()),
        Err(e) => Err(e),
    }
}

fn part_two(grid: &Grid<u8>) -> Result<usize> {
    match grid
        .rows()
        .iter()
        .enumerate()
        .map(|(i, row)| {
            match row
                .iter()
                .enumerate()
                .map(|(j, tree)| -> Result<_> {
                    let col = grid.col(j)?;
                    Ok(count_left(row, j, tree)
                        * count_right(row, j, tree)
                        * count_left(col, i, tree)
                        * count_right(col, i, tree))
                })
                .collect::<Result<Vec<_>>>()
            {
                Ok(v) => Ok(v.into_iter().max().unwrap()),
                Err(e) => Err(e),
            }
        })
        .collect::<Result<Vec<_>>>()
    {
        Ok(v) => Ok(v.into_iter().max().unwrap()),
        Err(e) => Err(e),
    }
}

fn visible_left(line: &[u8], idx: usize, tree: &u8) -> bool {
    line.iter().take(idx).all(|t| *t < *tree)
}

fn visible_right(line: &[u8], idx: usize, tree: &u8) -> bool {
    line.iter().skip(idx + 1).all(|t| *t < *tree)
}

fn count_left(line: &[u8], idx: usize, tree: &u8) -> usize {
    if idx != 0 {
        line.iter()
            .enumerate()
            .take(idx)
            .rev()
            .take_while(|(i, t)| **t < *tree && *i != 0)
            .count()
            + 1
    } else {
        0
    }
}

fn count_right(line: &[u8], idx: usize, tree: &u8) -> usize {
    if idx != line.len() - 1 {
        line.iter()
            .enumerate()
            .skip(idx + 1)
            .take_while(|(i, t)| **t < *tree && *i != line.len() - 1)
            .count()
            + 1
    } else {
        0
    }
}
