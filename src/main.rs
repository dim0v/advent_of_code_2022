use std::env;
use std::error::Error;

use anyhow::anyhow;
use advent_of_code_2022::solutions::get_solver_for_day;
use advent_of_code_2022::{process_input, Stage};

fn main() -> Result<(), Box<dyn Error>> {
    let args = parse_args()?;
    println!("day = {}, stage = {:?}", args.day, args.stage);

    let solver = get_solver_for_day(args.day);
    let result = solver(args.stage, &process_input(args.day));

    println!("{}", result);

    Ok(())
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
