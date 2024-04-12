use common::SolverFunc;
use seq_macro::seq;

mod common;

seq!(N in 1..=4 {
    
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
