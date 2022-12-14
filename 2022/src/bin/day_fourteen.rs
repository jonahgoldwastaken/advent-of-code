use anyhow::Result;
use itertools::Itertools;
use std::{default::Default, fmt::Display};

static SAND_START_X: usize = 500;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
	x: usize,
	y: usize,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum PointType {
	#[default]
	Air,
	Rock,
	Sand {
		falling: bool,
	},
}

impl Display for PointType {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			PointType::Air => write!(f, "."),
			PointType::Rock => write!(f, "#"),
			PointType::Sand { falling } => write!(f, "{}", if *falling { "O" } else { "0" }),
		}
	}
}

struct Cave {
	inner: Vec<Vec<PointType>>,
	x_start: usize,
	x_end: usize,
	y_end: usize,
}

impl Display for Cave {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for row in &self.inner {
			for point in row {
				write!(f, "{point}")?;
			}
			writeln!(f)?;
		}
		Ok(())
	}
}

impl Cave {
	fn new(x_start: usize, x_end: usize, y_end: usize) -> Self {
		Self {
			inner: vec![vec![Default::default(); x_end - x_start]; y_end],
			x_start,
			x_end,
			y_end,
		}
	}

	fn insert_extend(&mut self, x: usize, y: usize, point: PointType) {
		if self.x_start > x {
			for row in &mut self.inner {
				for _ in 0..(self.x_start - x) {
					row.insert(0, Default::default());
				}
			}
			self.x_start = x;
		} else if self.x_end - 1 < x {
			self.x_end = x + 1;
			for row in &mut self.inner {
				row.resize_with(self.x_end - self.x_start, Default::default);
			}
		}
		if self.y_end - 1 < y {
			self.resize_y(y + 1);
		}
		let test = x.overflowing_sub(self.x_start);
		if test.1 {
			println!("{x}, {}", self.x_start);
		}
		self.inner[y][x - self.x_start] = point;
	}

	fn resize_y(&mut self, depth: usize) {
		self.y_end = depth;
		self.inner.resize_with(self.y_end, || {
			vec![Default::default(); self.x_end - self.x_start]
		});
	}

	fn get(&self, x: usize, y: usize) -> Option<&PointType> {
		self.inner.get(y).and_then(|row| {
			let x = x.overflowing_sub(self.x_start);
			if x.1 {
				None
			} else {
				row.get(x.0)
			}
		})
	}

	fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut PointType> {
		self.inner.get_mut(y).and_then(|row| {
			let x = x.overflowing_sub(self.x_start);
			if x.1 {
				None
			} else {
				row.get_mut(x.0)
			}
		})
	}

	fn filtered_vec<FN>(&self, predicate: FN) -> Vec<PointType>
	where
		FN: Fn(PointType) -> bool,
	{
		self
			.inner
			.iter()
			.flat_map(|row| row.iter().filter(|point| predicate(**point)).cloned())
			.collect()
	}

	fn filter<FN>(&mut self, predicate: FN)
	where
		FN: Fn(&PointType) -> bool,
	{
		self.inner = self
			.inner
			.iter()
			.map(|row| {
				row
					.iter()
					.map(|p| if predicate(p) { *p } else { Default::default() })
					.collect()
			})
			.collect();
	}

	#[inline]
	fn boundaries(&self) -> (Coordinate, Coordinate) {
		(
			Coordinate {
				x: self.x_start,
				y: 0,
			},
			Coordinate {
				x: self.x_end - 1,
				y: self.y_end - 1,
			},
		)
	}
}

fn main() -> Result<()> {
	let input = aoc::load_input("fourteen")?;
	let rocks: Vec<(usize, usize)> = input
		.lines()
		.flat_map(|line| {
			line
				.split(" -> ")
				.map(|coordinate| {
					let (x, y) = coordinate.split(',').next_tuple().unwrap();
					(x.parse().unwrap(), y.parse().unwrap())
				})
				.tuple_windows::<((usize, usize), (usize, usize))>()
				.flat_map(|((x1, y1), (x2, y2))| {
					[
						if x1 > x2 { x2..=x1 } else { x1..=x2 }
							.map(|x| (x, y1))
							.collect_vec(),
						if y1 > y2 { y2..=y1 } else { y1..=y2 }
							.map(|y| (x1, y))
							.collect_vec(),
					]
					.concat()
				})
				.unique()
				.collect_vec()
		})
		.collect_vec();
	let x_start = rocks.iter().map(|(x, _)| x).min().unwrap();
	let x_end = rocks.iter().map(|(x, _)| x).max().unwrap() + 1;
	let y_end = rocks.iter().map(|(_, y)| y).max().unwrap() + 1;

	let mut cave = Cave::new(*x_start, x_end, y_end);
	for (x, y) in &rocks {
		cave.insert_extend(*x, *y, PointType::Rock);
	}
	simulate_sand(&mut cave, false);
	println!(
		"Part one: {}",
		cave
			.filtered_vec(|p| matches!(p, PointType::Sand { falling: false }))
			.len()
	);

	cave.filter(|p| !matches!(p, PointType::Sand { falling: true }));
	cave.resize_y(cave.boundaries().1.y + 3);
	simulate_sand(&mut cave, true);
	println!(
		"Part two: {}",
		cave
			.filtered_vec(|p| matches!(p, PointType::Sand { falling: false }))
			.len()
	);
	Ok(())
}

fn simulate_sand(cave: &mut Cave, cave_floor: bool) {
	let cave_dimensions = cave.boundaries();
	let y_end = cave_dimensions.1.y;
	loop {
		let mut sand_pos = Coordinate {
			x: SAND_START_X,
			y: 0,
		};
		while sand_pos.y < y_end {
			if cave_floor {
				if let Some(p) = cave.get_mut(sand_pos.x - 1, cave.boundaries().1.y) {
					if matches!(p, PointType::Air) {
						*p = PointType::Rock;
					}
				} else {
					cave.insert_extend(sand_pos.x - 1, cave.boundaries().1.y, PointType::Rock);
				}

				if let Some(p) = cave.get_mut(sand_pos.x, cave.boundaries().1.y) {
					if matches!(p, PointType::Air) {
						*p = PointType::Rock;
					}
				}

				if let Some(p) = cave.get_mut(sand_pos.x + 1, cave.boundaries().1.y) {
					if matches!(p, PointType::Air) {
						*p = PointType::Rock;
					}
				} else {
					cave.insert_extend(sand_pos.x + 1, cave.boundaries().1.y, PointType::Rock);
				}
			}

			if let Some(PointType::Air) = cave.get(sand_pos.x, sand_pos.y + 1) {
				sand_pos.y += 1;
			} else if let Some(PointType::Air) = cave.get(sand_pos.x - 1, sand_pos.y + 1) {
				sand_pos.x -= 1;
				sand_pos.y += 1;
			} else if let Some(PointType::Air) = cave.get(sand_pos.x + 1, sand_pos.y + 1) {
				sand_pos.x += 1;
				sand_pos.y += 1;
			} else {
				break;
			}
		}

		cave.insert_extend(
			sand_pos.x,
			sand_pos.y,
			PointType::Sand {
				falling: sand_pos.y >= y_end,
			},
		);

		if sand_pos.y >= y_end {
			break;
		} else if let Some(PointType::Sand { falling: false }) = cave.get(SAND_START_X, 0) {
			break;
		}
	}
}
