use std::cmp::max;
use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::str::FromStr;

use anyhow::anyhow;
use seq_macro::seq;

use crate::solutions::day9::Direction::{Down, Left, Right, Up};
use crate::Stage;

pub fn solve(stage: Stage, input: &str) -> String {
    let mut rope = Rope::new(match stage {
        Stage::Easy => 2,
        Stage::Hard => 10,
    });
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(*rope.get_tail_position());

    for row in input.lines() {
        let (dir, cnt) = row.split_at(1);

        let dir: Direction = dir.parse().unwrap();
        let cnt: i32 = cnt.trim_start().parse().unwrap();

        for _ in 0..cnt {
            rope.step(dir);
            visited.insert(*rope.get_tail_position());
        }
    }

    visited.len().to_string()
}

struct Rope {
    links: Vec<(i32, i32)>,
}

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Rope {
    fn new(len: usize) -> Rope {
        Rope {
            links: (0..len).map(|_| (0, 0)).collect(),
        }
    }

    fn get_tail_position(&self) -> &(i32, i32) {
        self.links.last().unwrap()
    }

    fn step(&mut self, dir: Direction) {
        let offset = match dir {
            Left => (-1, 0),
            Up => (0, 1),
            Right => (1, 0),
            Down => (0, -1),
        };
        
        add_offset(&mut self.links[0], offset);

        for i in 1..self.links.len() {
            let (head, tail) = {
                let (head, tail) = self.links.split_at_mut(i);
                (head.last_mut().unwrap(), tail.first_mut().unwrap())
            };

            let offset = {
                let mut v = (head.0 - tail.0, head.1 - tail.1);
                
                if max(v.0.abs(), v.1.abs()) < 2 { break; }

                seq!(N in 0..=1 {
                    if v.N.abs() == 2 {
                        v.N /= 2
                    }
                });

                v
            };

            add_offset(tail, offset);
        }
        
        #[inline]
        fn add_offset(target: &mut (i32, i32), offset: (i32, i32)) {
            target.0 += offset.0;
            target.1 += offset.1;
        }
    }

    #[allow(dead_code)]
    fn print(&self, range_x: RangeInclusive<i32>, range_y: RangeInclusive<i32>) {
        let mut field: Vec<Vec<char>> = range_y
            .clone()
            .map(|_| range_x.clone().map(|_| '.').collect())
            .collect();

        let offset = (-range_x.start(), -range_y.start());

        field[offset.1 as usize][offset.0 as usize] = 's';

        for (i, (x, y)) in self.links.iter().enumerate().rev() {
            field[(y + offset.1) as usize][(x + offset.0) as usize] = match i {
                0 => 'H',
                i => char::from_digit(i as u32, 10).unwrap(),
            };
        }

        for row in field.iter().rev() {
            println!("{}", String::from_iter(row.iter()));
        }
    }
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Left),
            "U" => Ok(Up),
            "R" => Ok(Right),
            "D" => Ok(Down),
            _ => Err(anyhow!("Invalid string")),
        }
    }
}
