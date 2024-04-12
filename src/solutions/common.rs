use crate::Stage;

pub type SolverFunc = fn(stage: Stage, input: &Vec<String>) -> String;
