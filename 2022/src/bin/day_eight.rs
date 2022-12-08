use anyhow::Result;
use itertools::Itertools;

struct Grid {
    rows: Vec<Vec<u8>>,
    cols: Vec<Vec<u8>>,
}

impl From<Vec<Vec<u8>>> for Grid {
    fn from(rows: Vec<Vec<u8>>) -> Self {
        let cols = (0..rows[0].len())
            .map(|i| rows.iter().map(|row| row[i]).collect())
            .collect();
        Self { rows, cols }
    }
}

impl Grid {
    fn rows(&self) -> &[Vec<u8>] {
        self.rows.as_ref()
    }

    fn cols(&self) -> &[Vec<u8>] {
        self.cols.as_ref()
    }

    fn col(&self, idx: usize) -> Option<&[u8]> {
        if idx < self.cols.len() {
            Some(&self.cols[idx])
        } else {
            None
        }
    }
}

fn main() -> Result<()> {
    let grid: Grid = Grid::from(
        aoc::load_input("eight")?
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| String::from(c).parse().unwrap())
                    .collect_vec()
            })
            .collect_vec(),
    );

    println!("Part one: {}", part_one(&grid));
    println!("Part two: {}", part_two(&grid));
    Ok(())
}

fn part_one(grid: &Grid) -> usize {
    grid.rows().iter().enumerate().fold(0, |res, (i, row)| {
        res + row.iter().enumerate().fold(0, |res, (j, tree)| {
            let col = grid.cols().get(j).unwrap();
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
    grid.rows()
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, tree)| {
                    let col = grid.col(j).unwrap();
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
