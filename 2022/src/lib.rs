use anyhow::{anyhow, Result};

pub fn load_input(day: &str) -> Result<String> {
	Ok(std::fs::read_to_string(format!("./input/day_{day}"))?)
}

#[derive(Debug, Clone)]
pub struct Grid<T> {
	rows: Vec<Vec<T>>,
	cols: Vec<Vec<T>>,
}

impl<T> From<Vec<Vec<T>>> for Grid<T>
where
	T: Sized + Clone + Copy,
{
	fn from(rows: Vec<Vec<T>>) -> Self {
		let cols = (0..rows[0].len())
			.map(|i| rows.iter().map(|row| row[i]).collect())
			.collect();
		Self { rows, cols }
	}
}

impl<T> Grid<T> {
	pub fn rows(&self) -> &[Vec<T>] {
		self.rows.as_ref()
	}

	pub fn row(&self, idx: usize) -> Result<&[T]> {
		if idx < self.rows.len() {
			Ok(&self.rows[idx])
		} else {
			Err(anyhow!("Invalid row index: {}", idx))
		}
	}

	pub fn col(&self, idx: usize) -> Result<&[T]> {
		if idx < self.cols.len() {
			Ok(&self.cols[idx])
		} else {
			Err(anyhow!("Invalid column index: {}", idx))
		}
	}
}
