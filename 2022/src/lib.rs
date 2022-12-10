use anyhow::Result;

pub fn load_input(day: &str) -> Result<String> {
    Ok(std::fs::read_to_string(format!("./input/day_{day}"))?)
}
