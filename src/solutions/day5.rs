use std::num::ParseIntError;
use std::str::FromStr;

use crate::Stage;

pub fn solve(stage: Stage, input: &str) -> String {
    let mut stacks = CrateStacks::from_strings(input.lines().take_while(|s| s.len() > 0).collect());

    let moves = input
        .lines()
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
    fn from_strings(ss: Vec<&str>) -> CrateStacks {
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
    
    fn get_stacks_for_op_mut(&mut self, op: &MoveOp) -> (&mut Vec<char>, &mut Vec<char>) {
        let (src, dst) = if op.from < op.to { 
            let (left, right) = self.stacks.split_at_mut(op.to);
            (&mut left[op.from], right.first_mut().unwrap())
        } else {
            let (left, right) = self.stacks.split_at_mut(op.from);
            (right.first_mut().unwrap(), &mut left[op.to])
        };

        (src, dst)
    }

    fn execute(&mut self, op: MoveOp, preserve_order: bool) {
        let (src, dst) = self.get_stacks_for_op_mut(&op);
        
        if preserve_order {
            let new_src_len = src.len() - op.count;
            let sub_stack = &src[new_src_len..];

            dst.extend_from_slice(sub_stack);
            src.truncate(new_src_len);
        } else {
            for _ in 0..op.count {
                dst.push(src.pop().unwrap())
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
