use seq_macro::seq;
use crate::Stage;

mod common;

seq!(N in 1..=16 {
    
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
    
    pub static INPUTS: &[&str] = &[
        #(
        include_str!(concat!("../inputs/day", stringify!(N), ".txt")),
        )*
    ];
});

pub type SolverFunc = fn(stage: Stage, input: &str) -> String;

pub static N_DAYS: u8 = INPUTS.len() as u8;
