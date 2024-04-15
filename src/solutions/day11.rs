use crate::Stage;
use anyhow::anyhow;
use std::str::FromStr;

pub fn solve(stage: Stage, input: &Vec<&str>) -> String {
    let mut monkeys: Vec<_> = input.chunks(7).map(Monkey::from_description).collect();

    let (iterations, relaxing_factor, modulus) = match stage {
        Stage::Easy => (20, 3, u64::MAX),
        Stage::Hard => (10_000, 1, monkeys.iter().map(|m| m.divisibility_test).product())
    };

    for _ in 0..iterations {
        simulate_round(&mut monkeys, relaxing_factor, modulus);
    }

    let busiest = max2(monkeys.iter().map(|m| m.inspected_cnt));

    (busiest.0 * busiest.1).to_string()
}

fn simulate_round(monkeys: &mut Vec<Monkey>, relaxing_factor: u64, modulus: u64) {
    for i in 0..monkeys.len() {
        let (monkey, (target_false, target_true)) = extract_monkeys(monkeys, i);
        while let Some(item) = monkey.items.pop() {
            monkey.inspected_cnt += 1;
            let new_value = (monkey.transform.apply(item) / relaxing_factor) % modulus;
            let test_result = new_value % monkey.divisibility_test == 0;

            let target = if test_result {
                &mut *target_true
            } else {
                &mut *target_false
            };
            target.items.push(new_value);
        }
    }
}

fn extract_monkeys(
    monkeys: &mut Vec<Monkey>,
    i: usize,
) -> (&mut Monkey, (&mut Monkey, &mut Monkey)) {
    let (left, right) = monkeys.split_at_mut(i);
    let (m, right) = right.split_first_mut().unwrap();
    let target_idx_left = &m.target_idx;
    let target_idx_right = target_idx_left.map(|x| x.saturating_sub(i + 1));

    let pair = if m.target_idx[0] > i && m.target_idx[1] > i {
        extract_pair(right, target_idx_right[0], target_idx_right[1])
    } else if m.target_idx[0] < i && m.target_idx[1] < i {
        extract_pair(left, target_idx_left[0], target_idx_left[1])
    } else if m.target_idx[0] < i && m.target_idx[1] > i {
        (left.get_mut(target_idx_left[0]).unwrap(), right.get_mut(target_idx_right[1]).unwrap())
    } else if m.target_idx[0] > i && m.target_idx[1] < i {
        (right.get_mut(target_idx_right[0]).unwrap(), left.get_mut(target_idx_left[1]).unwrap())
    } else { 
        panic!("welp.. too bad")
    };

    fn extract_pair(
        monkeys: &mut [Monkey],
        first: usize,
        second: usize,
    ) -> (&mut Monkey, &mut Monkey) {
        assert_ne!(first, second);

        if first < second {
            let (left, right) = monkeys.split_at_mut(second);
            (&mut left[first], right.first_mut().unwrap())
        } else {
            let (left, right) = monkeys.split_at_mut(first);
            (right.first_mut().unwrap(), &mut left[second])
        }
    }

    (m, pair)
}

fn max2<T: Ord>(mut iter: impl Iterator<Item = T>) -> (T, T) {
    let (mut a, mut b) = (iter.next().unwrap(), iter.next().unwrap());

    if a > b {
        (a, b) = (b, a)
    }

    for i in iter {
        if i > b {
            (a, b) = (b, i)
        } else if i > a {
            a = i
        }
    }

    (a, b)
}

struct Monkey {
    items: Vec<u64>,
    transform: Transform,
    divisibility_test: u64,
    target_idx: [usize; 2], // 0'th element for failed test, 1'st element for passed test

    inspected_cnt: usize,
}

struct Transform {
    op: Operation,
    arg1: Operand,
    arg2: Operand,
}

impl Transform {
    fn apply(&self, item: u64) -> u64 {
        let [arg1, arg2] = [&self.arg1, &self.arg2].map(|x| x.get_value(item));

        match self.op {
            Operation::Add => arg1 + arg2,
            Operation::Mul => arg1 * arg2,
        }
    }
}

enum Operation {
    Add,
    Mul,
}

enum Operand {
    Current,
    Imm(u64),
}

impl Operand {
    fn get_value(&self, current: u64) -> u64 {
        match self {
            Operand::Current => current,
            Operand::Imm(v) => *v,
        }
    }
}

impl Monkey {
    fn from_description(descr: &[&str]) -> Monkey {
        assert!(descr.len() >= 6);

        Monkey {
            items: descr[1]
                .split_at(18)
                .1
                .split(", ")
                .map(|x| x.parse().unwrap())
                .collect(),
            transform: descr[2].split_at(19).1.parse().unwrap(),
            divisibility_test: descr[3].split_at(21).1.parse().unwrap(),
            target_idx: [
                descr[5].split_at(30).1.parse().unwrap(),
                descr[4].split_at(29).1.parse().unwrap(),
            ],
            inspected_cnt: 0,
        }
    }
}

impl FromStr for Operand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(Operand::Current),
            s => Ok(Operand::Imm(s.parse()?)),
        }
    }
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Operation::Mul),
            "+" => Ok(Operation::Add),
            op => Err(anyhow!("Invalid op: {op}")),
        }
    }
}

impl FromStr for Transform {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');

        Ok(Transform {
            arg1: parts.next().unwrap().parse()?,
            op: parts.next().unwrap().parse()?,
            arg2: parts.next().unwrap().parse()?,
        })
    }
}
