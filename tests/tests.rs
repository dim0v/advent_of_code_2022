use seq_macro::seq;

use advent_of_code_2022::solutions::{get_solver_for_day, INPUTS};
use advent_of_code_2022::*;

static ANSWERS: &[[&str; 2]] = &[
    ["69795", "208437"],
    ["13268", "15508"],
    ["7785", "2633"],
    ["569", "936"],
    ["RLFNRTNFB", "MHQTLJRLB"],
    ["1582", "3588"],
    ["1908462", "3979145"],
    ["1719", "590824"],
    ["5779", "2331"],
    [
        "14320",
        "\
▉▉▉   ▉▉  ▉▉▉  ▉▉▉  ▉  ▉  ▉▉  ▉▉▉    ▉▉ 
▉  ▉ ▉  ▉ ▉  ▉ ▉  ▉ ▉ ▉  ▉  ▉ ▉  ▉    ▉ 
▉  ▉ ▉    ▉  ▉ ▉▉▉  ▉▉   ▉  ▉ ▉  ▉    ▉ 
▉▉▉  ▉    ▉▉▉  ▉  ▉ ▉ ▉  ▉▉▉▉ ▉▉▉     ▉ 
▉    ▉  ▉ ▉    ▉  ▉ ▉ ▉  ▉  ▉ ▉    ▉  ▉ 
▉     ▉▉  ▉    ▉▉▉  ▉  ▉ ▉  ▉ ▉     ▉▉  
 ",
    ],
    ["151312", "51382025916"],
    ["481", "480"],
    ["4821", "21890"],
    ["1406", "20870"],
    ["4582667", "10961118625406"],
    ["1559", "2191"],
];

seq!(N in 1..=25 {
    #[test]
    fn easy_day~N() {
        if N > ANSWERS.len() {
            println!("Not solved yet, skipping");
        } else {
            assert_eq!(ANSWERS[N - 1][0], compute_answer(N, Stage::Easy));
        }
    }

    #[test]
    fn hard_day~N() {
        if N > ANSWERS.len() {
            println!("Not solved yet, skipping");
        } else {
            assert_eq!(ANSWERS[N - 1][1], compute_answer(N, Stage::Hard));
        }
    }
});

#[ignore]
fn compute_answer(day: u8, stage: Stage) -> String {
    get_solver_for_day(day)(stage, INPUTS[(day - 1) as usize])
}
