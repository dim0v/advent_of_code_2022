use std::num::ParseIntError;
use std::str::FromStr;

use crate::Stage;

pub fn solve(stage: Stage, input: Vec<String>) -> String {
    let mut stacks = CrateStacks::from_strings(input.iter().take_while(|s| s.len() > 0).collect());

    let moves = input
        .iter()
        .skip_while(|s| s.len() > 0)
        .skip(1)
        .map(|s| s.parse::<MoveOp>().unwrap());

    for m in moves {
        stacks.execute(m, stage.is_hard());
    }

    let top_crates: Vec<String> = stacks
        .stacks
        .iter()
        .map(|x| x.last().or(Some(&'\0')).unwrap().to_string())
        .collect();

    top_crates.join("")
}

struct CrateStacks {
    stacks: Vec<Vec<char>>,
}

impl CrateStacks {
    fn from_strings(ss: Vec<&String>) -> CrateStacks {
        let cnt = (ss[0].len() + 1) / 4;

        let mut stacks = Vec::with_capacity(cnt);

        for _ in 0..cnt {
            stacks.push(Vec::new())
        }

        for s in ss.iter().rev().skip(1) {
            for (i, char) in s.chars().skip(1).step_by(4).enumerate() {
                if char.is_alphabetic() {
                    stacks[i].push(char)
                }
            }
        }

        CrateStacks { stacks }
    }

    fn execute(&mut self, op: MoveOp, preserve_order: bool) {
        if preserve_order {
            let new_src_len = self.stacks[op.from].len() - op.count;
            let sub_stack = self.stacks[op.from][new_src_len..].to_vec();

            self.stacks[op.to].extend_from_slice(sub_stack.as_slice());
            self.stacks[op.from].truncate(new_src_len);
        } else {
            for _ in 0..op.count {
                let val = self.stacks[op.from].pop().unwrap();
                self.stacks[op.to].push(val)
            }
        }
    }
}

struct MoveOp {
    from: usize,
    to: usize,
    count: usize,
}

impl FromStr for MoveOp {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ').skip(1).step_by(2).map(|s| s.parse::<usize>());

        Ok(MoveOp {
            count: parts.next().unwrap()?,
            from: parts.next().unwrap()? - 1,
            to: parts.next().unwrap()? - 1,
        })
    }
}
