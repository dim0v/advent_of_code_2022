use crate::Stage;

pub fn solve(stage: Stage, input: Vec<String>) -> i128 {
    match stage {
        Stage::Easy => { input[0].len() as i128 }
        Stage::Hard => { input[1].len() as i128 }
    }
}
