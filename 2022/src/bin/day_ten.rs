use anyhow::{bail, Result};

fn main() -> Result<()> {
    let input = aoc::load_input("ten")?;
    let mut cycle: u32 = 0;
    let mut register: i32 = 1;
    let mut signal_strengths: Vec<i32> = vec![];
    let mut crt: [[bool; 40]; 6] = [[false; 40]; 6];
    for l in input.lines() {
        match l.split(' ').next().unwrap_or("") {
            "noop" => {
                cycle += 1;
                clock_cycle(cycle, register, &mut signal_strengths, &mut crt);
            }
            "addx" => {
                cycle += 1;
                clock_cycle(cycle, register, &mut signal_strengths, &mut crt);
                cycle += 1;
                register += l.split(' ').last().unwrap().parse::<i32>().unwrap();
                clock_cycle(cycle, register, &mut signal_strengths, &mut crt);
            }
            _ => bail!("Invalid line"),
        };
    }
    println!("Part one: {}", signal_strengths.iter().sum::<i32>());
    println!("Part two: |");
    println!("          v");
    print_crt(&crt);

    Ok(())
}

fn clock_cycle(
    cycle: u32,
    register: i32,
    signal_strengths: &mut Vec<i32>,
    crt: &mut [[bool; 40]; 6],
) {
    handle_signal_strengths(cycle + 1, register, signal_strengths);

    let idx = crt_index(cycle);
    crt[idx.0][idx.1] = (register - 1..=register + 1).contains(&(idx.1 as i32));
}

fn handle_signal_strengths(cycle: u32, register: i32, signal_strengths: &mut Vec<i32>) {
    match cycle {
        20 => signal_strengths.push(register * cycle as i32),
        60 => signal_strengths.push(register * cycle as i32),
        100 => signal_strengths.push(register * cycle as i32),
        140 => signal_strengths.push(register * cycle as i32),
        180 => signal_strengths.push(register * cycle as i32),
        220 => signal_strengths.push(register * cycle as i32),
        _ => (),
    };
}

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
