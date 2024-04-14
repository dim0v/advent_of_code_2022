use std::str::FromStr;

use anyhow::anyhow;

use crate::Stage;

pub fn solve(stage: Stage, input: &Vec<&str>) -> String {
    let mut vm = Vm::new();

    let total_strength: i64 = vm
        .execute_program(input)
        .enumerate()
        .skip(18)
        .step_by(40)
        .map(|(i, st)| ((i + 2) as i64) * st.x)
        .sum();

    total_strength.to_string()
}

#[derive(Copy, Clone)]
struct VmState {
    x: i64,
}

struct Vm {
    state: VmState,
}

impl Vm {
    fn new() -> Vm {
        Vm {
            state: VmState { x: 1 },
        }
    }

    fn execute(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Noop => {}
            Instruction::Addx(v) => self.state.x += v,
        }
    }

    fn execute_program(&mut self, program: &[&str]) -> VmExecutionIterator {
        VmExecutionIterator::new(self, program)
    }
}

#[derive(Copy, Clone)]
enum Instruction {
    Noop,
    Addx(i64),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(' ');

        match iter.next().ok_or(anyhow!("empty command"))? {
            "noop" => Ok(Instruction::Noop),
            "addx" => Ok(Instruction::Addx(
                iter.next().ok_or(anyhow!("missing argument"))?.parse()?,
            )),
            unknown_cmd => Err(anyhow!("Unknown instruction: {unknown_cmd}")),
        }
    }
}

impl Instruction {
    fn get_delay(&self) -> usize {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }
}

struct VmExecutionIterator<'a> {
    vm: &'a mut Vm,
    program: Vec<Instruction>,

    ip: usize,
    remaining_delay: usize,
}

impl<'a> VmExecutionIterator<'a> {
    fn new(vm: &'a mut Vm, program: &[&str]) -> VmExecutionIterator<'a> {
        let program: Vec<Instruction> = program.iter().map(|x| x.parse().unwrap()).collect();

        let delay = match program.get(0) {
            None => 0,
            Some(instr) => instr.get_delay(),
        };

        VmExecutionIterator {
            vm,
            ip: 0,
            remaining_delay: delay,
            program,
        }
    }
}

impl<'a> Iterator for VmExecutionIterator<'a> {
    // I give up arguing with the borrow checker. Apparently there's no way to have an immutable reference here...
    type Item = VmState;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining_delay > 0 {
            self.remaining_delay -= 1;

            if self.remaining_delay > 0 {
                return Some(self.vm.state);
            }
        }

        let cmd_current = self.program.get(self.ip)?;
        self.vm.execute(cmd_current);
        self.ip += 1;

        let cmd_next = self.program.get(self.ip);
        if let Some(cmd_next) = cmd_next {
            self.remaining_delay = cmd_next.get_delay();
        }

        Some(self.vm.state)
    }
}

