use common::SolverFunc;
use seq_macro::seq;

mod common;

seq!(N in 1..=7 {
    
    #(
        mod day~N;
    )*

    pub fn get_solver_for_day(day: u8) -> SolverFunc {
        match day {
            #(
            N => day~N::solve,
            )*
            _ => panic!("Invalid day"),
        }
    }
});

pub static INPUTS: &[&str] = &[
    include_str!("../inputs/day1.txt"),
    include_str!("../inputs/day2.txt"),
    include_str!("../inputs/day3.txt"),
    include_str!("../inputs/day4.txt"),
    include_str!("../inputs/day5.txt"),
    include_str!("../inputs/day6.txt"),
    include_str!("../inputs/day7.txt"),
];
