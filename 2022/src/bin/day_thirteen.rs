use std::{cell::RefCell, cmp::Ordering, rc::Rc};

use anyhow::{anyhow, Result};
use itertools::{EitherOrBoth, Itertools};

macro_rules! rc {
	($e:expr) => {
		Rc::new(RefCell::new($e))
	};
}

type RcNode = Rc<RefCell<Node>>;

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
enum Data {
	List(Vec<RcNode>),
	Int(usize),
	None,
}

impl std::fmt::Display for Data {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::List(list) => {
				write!(f, "[")?;
				list.iter().enumerate().try_for_each(|(idx, val)| {
					val.borrow().fmt(f)?;
					if idx < list.len() - 1 {
						write!(f, ",")?;
					}
					Ok(())
				})?;
				write!(f, "]")
			}
			Self::Int(val) => write!(f, "{val}"),
			Self::None => write!(f, ""),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
struct Node {
	#[serde(skip_serializing)]
	parent: Option<RcNode>,
	value: Data,
}

impl PartialOrd for Node {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		self.cmp(other).into()
	}
}

impl Ord for Node {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		let left = if let Data::List(v) = &self.value {
			v
		} else {
			return Ordering::Equal;
		};
		let right = if let Data::List(v) = &other.value {
			v
		} else {
			return Ordering::Equal;
		};
		let mut ordered = Ordering::Equal;
		for eob in left.iter().zip_longest(right.iter()) {
			ordered = match eob {
				EitherOrBoth::Both(left, right) => {
					let l = left.borrow();
					let r = right.borrow();
					match (&l.value, &r.value) {
						(Data::Int(l), Data::Int(r)) if l == r => Ordering::Equal,
						(Data::Int(l), Data::Int(r)) if l < r => Ordering::Less,
						(Data::Int(_), Data::Int(_)) => Ordering::Greater,
						(Data::List(_), Data::List(_)) => left.borrow().cmp(&right.borrow()),
						(Data::Int(l), Data::List(_)) => rc!(Node {
							parent: None,
							value: Data::List(vec![rc!(Node {
								parent: None,
								value: Data::Int(*l),
							})])
						})
						.borrow()
						.cmp(&right.borrow()),
						(Data::List(_), Data::Int(r)) => left.borrow().cmp(
							&rc!(Node {
								parent: None,
								value: Data::List(vec![rc!(Node {
									parent: None,
									value: Data::Int(*r),
								})])
							})
							.borrow(),
						),
						_ => unreachable!(),
					}
				}
				EitherOrBoth::Left(_) => Ordering::Greater,
				EitherOrBoth::Right(_) => Ordering::Less,
			};
			if ordered != Ordering::Equal {
				break;
			}
		}
		ordered
	}
}

impl std::fmt::Display for Node {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.value.fmt(f)
	}
}

fn main() -> Result<()> {
	let input = aoc::load_input("thirteen")?;
	let t: Vec<(RcNode, RcNode)> = input
		.split("\n\n")
		.map(|pair| {
			let mut lines = pair.lines();
			Ok((
				create_node(lines.next().unwrap())?,
				create_node(lines.next().unwrap())?,
			))
		})
		.collect::<Result<_>>()?;

	println!(
		"Part one: {}",
		t.iter()
			.enumerate()
			.filter(|(_, d)| match d.0.borrow().cmp(&d.1.borrow()) {
				Ordering::Less => true,
				Ordering::Greater => false,
				_ => false,
			})
			.map(|(i, _)| i + 1)
			.sum::<usize>()
	);

	println!(
		"Part two: {}",
		t.iter()
			.flat_map(|(d0, d1)| [d0, d1])
			.chain([create_node("[[2]]")?, create_node("[[6]]")?].iter())
			.sorted()
			.enumerate()
			.filter_map(|(i, d)| {
				if format!("{}", d.borrow()) == "[[2]]" || format!("{}", d.borrow()) == "[[6]]" {
					Some(i + 1)
				} else {
					None
				}
			})
			.product::<usize>()
	);

	Ok(())
}

fn create_node(line: &str) -> Result<RcNode> {
	let mut data = line.chars().skip(1).take(line.chars().count() - 2).fold(
		Ok(rc!(Node {
			parent: None,
			value: Data::List(vec![])
		})),
		|current_data, c| {
			let current_data = current_data?;
			match c {
				'[' => {
					let new_data = rc!(Node {
						parent: Some(current_data.clone()),
						value: Data::List(vec![])
					});
					let mut d = current_data.borrow_mut();
					if let Data::List(ref mut v) = d.value {
						if let Some(last) = v.last_mut() {
							if matches!(last.clone().borrow().value, Data::None) {
								*last = new_data.clone();
							} else {
								v.push(new_data.clone());
							}
						} else {
							v.push(new_data.clone());
						}
						Ok(new_data)
					} else {
						return Err(anyhow!("Expected list for new list"));
					}
				}
				']' => {
					if let Some(parent) = current_data.borrow().parent.clone() {
						Ok(parent)
					} else {
						return Err(anyhow!("Expected current_node to have parent"));
					}
				}
				',' => {
					{
						if let Data::List(ref mut v) = current_data.borrow_mut().value {
							v.push(rc!(Node {
								parent: None,
								value: Data::None
							}));
						} else {
							return Err(anyhow!("Expected list for comma separator"));
						}
					}
					Ok(current_data)
				}
				x if x.to_string().parse::<usize>().is_ok() => {
					let i = x.to_string().parse::<usize>().unwrap();

					let mut d = current_data.borrow_mut();
					if let Data::List(ref mut list) = d.value {
						let value = rc!(Node {
							parent: Some(current_data.clone()),
							value: Data::Int(i),
						});
						if let Some(last) = list.last_mut() {
							let last_borrow = last.clone();
							let last_borrow = last_borrow.borrow();
							match last_borrow.value {
								Data::None => {
									*last = value;
								}
								Data::Int(v) => {
									value.borrow_mut().value = Data::Int(v * 10 + i);
									*last = value
								}
								_ => list.push(value),
							}
						} else {
							list.push(value);
						}
						std::mem::drop(d);
						Ok(current_data)
					} else {
						return Err(anyhow!("Expected list for integer"));
					}
				}
				_ => unreachable!(),
			}
		},
	)?;
	while let Some(p) = data.clone().borrow().parent.clone() {
		data = p;
	}
	Ok(data)
}
