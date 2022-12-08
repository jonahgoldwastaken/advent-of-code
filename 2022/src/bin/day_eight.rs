use anyhow::Result;

type Grid = Vec<Vec<u8>>;

fn main() -> Result<()> {
    let grid: Grid = aoc::load_input("eight")?
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| String::from(c).parse().unwrap())
                .collect()
        })
        .collect();

    println!("Part one: {}", part_one(&grid));
    println!("Part two: {}", part_two(&grid));
    Ok(())
}

fn part_one(grid: &Grid) -> usize {
    let mut columns: Vec<Vec<u8>> = vec![];
    grid.iter().enumerate().fold(0, |res, (i, row)| {
        res + row.iter().enumerate().fold(0, |res, (j, tree)| {
            if columns.get(j).is_none() {
                columns.push(get_column(grid, j).unwrap());
            }
            let col = columns.get(j).unwrap();
            if visible_left(row, j, tree)
                || visible_right(row, j, tree)
                || visible_left(col, i, tree)
                || visible_right(col, i, tree)
            {
                res + 1
            } else {
                res
            }
        })
    })
}

fn part_two(grid: &Grid) -> usize {
    let mut columns: Vec<Vec<u8>> = vec![];
    grid.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, tree)| {
                    if columns.get(j).is_none() {
                        columns.push(get_column(grid, j).unwrap());
                    }
                    let col = columns.get(j).unwrap();
                    count_left(row, j, tree)
                        * count_right(row, j, tree)
                        * count_left(col, i, tree)
                        * count_right(col, i, tree)
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
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

fn get_column(grid: &Grid, col: usize) -> Option<Vec<u8>> {
    if grid[0].len() < col {
        None
    } else {
        Some(grid.iter().map(|row| row[col]).collect())
    }
}
