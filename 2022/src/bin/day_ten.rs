use anyhow::{bail, Result};

fn main() -> Result<()> {
	let input = aoc::load_input("ten")?;
	let mut cycle: u32 = 0;
	let mut register: i32 = 1;
	let mut signal_strengths: i32 = 0;
	let mut crt: [[bool; 40]; 6] = [[false; 40]; 6];
	for l in input.lines() {
		match l.split(' ').next().unwrap_or("") {
			"noop" => {
				cycle = clock_cycle(cycle, register, &mut signal_strengths, &mut crt);
			}
			"addx" => {
				cycle = clock_cycle(cycle, register, &mut signal_strengths, &mut crt);
				register += l.split(' ').last().unwrap().parse::<i32>().unwrap();
				cycle = clock_cycle(cycle, register, &mut signal_strengths, &mut crt);
			}
			_ => bail!("Invalid line"),
		};
	}
	println!("Part one: {signal_strengths}");
	println!("Part two: \u{2193}");
	print_crt(&crt);

	Ok(())
}

fn clock_cycle(
	cycle: u32,
	register: i32,
	signal_strengths: &mut i32,
	crt: &mut [[bool; 40]; 6],
) -> u32 {
	let current_cycle = cycle + 1;

	// Part one
	if (current_cycle + 1) % 40 == 20 {
		*signal_strengths += register * (current_cycle + 1) as i32;
	}

	// Part two
	let idx = crt_index(current_cycle);
	crt[idx.0][idx.1] = (register - 1..=register + 1).contains(&(idx.1 as i32));
	current_cycle
}

#[inline]
fn crt_index(cycle: u32) -> (usize, usize) {
	(cycle as usize / 40 % 6, cycle as usize % 40)
}

fn print_crt(crt: &[[bool; 40]; 6]) {
	for row in crt {
		for pixel in row {
			if *pixel {
				print!("#");
			} else {
				print!(".");
			}
		}
		println!();
	}
}
