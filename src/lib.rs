use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use solutions::INPUTS;

pub mod solutions;

pub fn process_input(day: u8) -> Vec<&'static str> {
    let data = INPUTS[day as usize - 1];

    data.split('\n').collect()
}

#[derive(Debug, Copy, Clone)]
pub enum Stage {
    Easy = 0,
    Hard = 1,
}

impl Display for Stage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        (&self as &dyn Debug).fmt(f)
    }
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
