use crate::Stage;

pub type SolverFunc = fn(stage: Stage, input: Vec<String>) -> i64;
