use crate::solutions::day2::MoveType::{Paper, Rock, Scissors};
use crate::Stage;
use anyhow::anyhow;
use std::str::FromStr;

pub fn solve(stage: Stage, input: Vec<String>) -> String {
    let mut rounds = Vec::from_iter(input.iter().map(|v| v.parse::<Round>().unwrap()));

    if stage.is_hard() {
        for round in &mut rounds {
            round.fix_my_move()
        }
    }

    let result: i64 = rounds.iter().map(|x| x.get_full_score()).sum();
    result.to_string()
}

#[derive(Copy, Clone)]
enum MoveType {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

struct Round {
    opponent_move: MoveType,
    my_move: MoveType,
}

impl MoveType {
    fn get_score(&self) -> i64 {
        match *self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

impl Round {
    fn get_round_score(&self) -> i64 {
        let scores = [[3, 6, 0], [0, 3, 6], [6, 0, 3]];

        scores[self.opponent_move as usize][self.my_move as usize]
    }

    fn get_full_score(&self) -> i64 {
        self.get_round_score() + self.my_move.get_score()
    }

    fn fix_my_move(&mut self) {
        let scores = [
            [Scissors, Rock, Paper],
            [Rock, Paper, Scissors],
            [Paper, Scissors, Rock],
        ];

        self.my_move = scores[self.opponent_move as usize][self.my_move as usize]
    }
}

impl FromStr for MoveType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Rock),
            "B" | "Y" => Ok(Paper),
            "C" | "Z" => Ok(Scissors),
            &_ => Err(anyhow!("Couldn't parse move")),
        }
    }
}

impl FromStr for Round {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = Vec::from_iter(s.split(' '));

        if parts.len() != 2 {
            return Err(anyhow!("Couldn't parse round"));
        }

        Ok(Round {
            opponent_move: parts[0].parse()?,
            my_move: parts[1].parse()?,
        })
    }
}
