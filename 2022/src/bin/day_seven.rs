use std::{
	cell::{Ref, RefCell},
	rc::Rc,
};

use anyhow::{bail, Result};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
	static ref START_RE: Regex = Regex::new(r"(^\S+)\s").unwrap();
	static ref CMD_RE: Regex = Regex::new(r"^\$ ([a-z]+)").unwrap();
	static ref FILE_RE: Regex = Regex::new(r"^(?P<size>\d+) (?P<name>.*)$").unwrap();
	static ref DIR_RE: Regex = Regex::new(r"^dir (.*)$").unwrap();
}

#[derive(Debug, Clone)]
struct Directory {
	parent: Option<Rc<RefCell<Directory>>>,
	#[allow(dead_code)]
	name: String,
	size: u64,
	children: Vec<Rc<RefCell<Directory>>>,
}

impl Directory {
	fn get_smallest_dir_size_for_size(&self, size: u64) -> Option<u64> {
		let mut result = None;
		if self.size >= size {
			result = Some(self.size);
		}

		for d in &self.children {
			let d_result = d.borrow().get_smallest_dir_size_for_size(size);
			if let Some(d_res) = d_result {
				match result {
					Some(res) => {
						if d_res > size && res > d_res {
							result = d_result
						}
					}
					None => {
						if d_res > size {
							result = d_result
						}
					}
				}
			}
		}
		result
	}
}

fn generate_dir_tree(
	input: String,
	parent: Option<Rc<RefCell<Directory>>>,
) -> Result<Rc<RefCell<Directory>>> {
	let mut lines = input.lines();
	let mut current_dir = Rc::new(RefCell::new(Directory {
		parent,
		name: lines.next().unwrap().split(' ').last().unwrap().to_string(),
		size: 0,
		children: vec![],
	}));
	for l in lines {
		let start = START_RE.captures(l).unwrap().get(1).unwrap();
		match start.as_str() {
			"$" => {
				let cmd = CMD_RE.captures(l).unwrap().get(1).unwrap();
				match cmd.as_str() {
					"cd" => {
						let dir_name = l.split(' ').nth(2).unwrap();
						if dir_name == ".." {
							if let Some(parent) = current_dir.clone().borrow().parent.clone() {
								current_dir = parent;
							} else {
								bail!("No parent directory");
							};
						} else {
							let new_dir = Rc::new(RefCell::new(Directory {
								parent: Some(current_dir.clone()),
								name: l.split(' ').last().unwrap().to_string(),
								size: 0,
								children: vec![],
							}));
							current_dir.borrow_mut().children.push(new_dir.clone());
							current_dir = new_dir;
						}
					}
					"ls" => continue,
					_ => unreachable!(),
				}
			}
			"dir" => continue,
			_ => match start.as_str().parse::<u64>() {
				Ok(fs) => {
					current_dir.borrow_mut().size += fs;
					let mut parent = current_dir.borrow().parent.clone();
					while let Some(p) = parent {
						p.borrow_mut().size += fs;
						parent = p.borrow().parent.clone();
					}
				}
				Err(_) => unreachable!(),
			},
		}
	}
	let mut parent = current_dir;
	while let Some(p) = parent.clone().borrow().parent.clone() {
		parent = p;
	}
	Ok(parent)
}

fn get_total_size(dir: Ref<Directory>, dir_size_limit: u64) -> u64 {
	let mut total = 0;
	if dir.size <= dir_size_limit {
		total += dir.size;
	}

	for d in dir.children.iter() {
		total += get_total_size(d.borrow(), dir_size_limit);
	}
	total
}

fn get_space_needed(dir: Ref<Directory>, total_space: u64, space_required: u64) -> u64 {
	space_required - (total_space - dir.size)
}

fn get_smallest_directory_needed(dir: Ref<Directory>, required: u64) -> u64 {
	dir
		.children
		.iter()
		.map(|d| d.borrow().get_smallest_dir_size_for_size(required))
		.map(|o| if let Some(o) = o { o } else { u64::MAX })
		.min()
		.unwrap()
}

fn main() -> Result<()> {
	let input = aoc::load_input("seven")?;
	let tree = generate_dir_tree(input, None)?;
	println!("Part one: {}", get_total_size(tree.borrow(), 100000));
	println!(
		"Part two: {}",
		get_smallest_directory_needed(
			tree.borrow(),
			get_space_needed(tree.borrow(), 70000000, 30000000)
		)
	);
	Ok(())
}
