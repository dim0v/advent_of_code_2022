use std::{env, io};
use std::error::Error;
use std::str::FromStr;

use anyhow::anyhow;

use crate::solutions::get_solver_for_day;

mod solutions;

fn main() -> Result<(), Box<dyn Error>> {
    let args = parse_args()?;
    println!("day = {}, stage = {:?}", args.day, args.stage);

    let solver = get_solver_for_day(args.day);
    let result = solver(args.stage, read_input(args.day)?);

    println!("{}", result);

    Ok(())
}

#[derive(Debug)]
enum Stage {
    Easy,
    Hard,
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

struct Args {
    day: u8,
    stage: Stage,
}

fn parse_args() -> Result<Args, Box<dyn Error>> {
    if env::args().count() < 2 {
        let err = anyhow!("Day argument is required");
        return Err(err.into());
    }

    let mut args = env::args().skip(1);

    let day: u8 = args.next().unwrap().parse()?;
    let stage: Stage = args.next().or(Some("easy".into())).unwrap().parse()?;

    Ok(Args { day, stage })
}

fn read_input(day: u8) -> io::Result<Vec<String>> {
    let file_name = format!("inputs/day{}.txt", day);
    let data = std::fs::read_to_string(file_name)?;

    let split = data.split("\n");

    Ok(Vec::from_iter(split.map(|x| String::from(x.trim()))))
}
