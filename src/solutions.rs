use common::SolverFunc;

mod common;
pub mod day1;

pub fn get_solver_for_day(day: u8) -> SolverFunc {
    match day {
        1 => day1::solve,
        _ => panic!("Invalid day"),
    }
}
