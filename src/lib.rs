use std::io;
use std::str::FromStr;

pub mod solutions;

pub fn read_input(day: u8) -> io::Result<Vec<String>> {
    let file_name = format!("inputs/day{}.txt", day);
    let data = std::fs::read_to_string(file_name)?;

    let split = data.split("\n");

    let trimmed = split
        .map(|x| String::from(x.trim_end_matches(|c| c == '\n' || c == '\r')));
    Ok(Vec::from_iter(trimmed))
}

#[derive(Debug)]
pub enum Stage {
    Easy = 0,
    Hard = 1,
}

impl Stage {
    fn is_hard(&self) -> bool {
        match self {
            Stage::Hard => true,
            Stage::Easy => false,
        }
    }
}

impl FromStr for Stage {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" | "hard" | "true" => Ok(Stage::Hard),
            &_ => Ok(Stage::Easy),
        }
    }
}