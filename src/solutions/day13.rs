use crate::solutions::day13::Node::{Int, List};
use crate::solutions::day13::Token::{BrClose, BrOpen, Comma, Invalid, Num};
use crate::Stage;
use anyhow::anyhow;
use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use std::str::FromStr;

pub fn solve(stage: Stage, input: &str) -> String {
    let mut parsed: Vec<Node> = input
        .lines()
        .filter_map(|r| {
            if r.len() > 0 {
                Some(r.parse().unwrap())
            } else {
                None
            }
        })
        .collect();

    match stage {
        Stage::Easy => solve_easy(parsed),
        Stage::Hard => solve_hard(&mut parsed),
    }
    .to_string()
}

fn solve_hard(packets: &mut Vec<Node>) -> usize {
    let p1: Node = "[[2]]".parse().unwrap();
    let p2: Node = "[[6]]".parse().unwrap();
    
    packets.push(p1.clone());
    packets.push(p2.clone());

    packets.sort();

    let p1 = packets
        .iter()
        .position(|it| *it == p1)
        .unwrap();
    let p2 = packets
        .iter()
        .position(|it| *it == p2)
        .unwrap();

    (p1 + 1) * (p2 + 1)
}

fn solve_easy(packets: Vec<Node>) -> usize {
    let filter_map: Vec<_> = packets
        .chunks(2)
        .enumerate()
        .map(|(i, ch)| if ch[0] <= ch[1] { Some(i + 1) } else { None })
        .collect();

    let cnt: usize = filter_map.iter().filter_map(|x| *x).sum();
    cnt
}

#[derive(Debug, Clone)]
enum Node {
    List(Vec<Node>),
    Int(i32),
}

impl Eq for Node {}

impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Equal
    }
}

impl PartialOrd<Self> for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return match self {
            Int(int_self) => match other {
                Int(int_other) => int_self.partial_cmp(int_other),
                _ => List(vec![Int(*int_self)]).partial_cmp(other),
            },
            List(list_self) => match other {
                Int(int_other) => {
                    Some(List(vec![Int(*int_other)]).cmp(self).reverse())
                }
                List(list_other) => {
                    for (n_self, n_other) in list_self.iter().zip(list_other) {
                        let cmp = n_self.cmp(n_other);
                        if cmp != Equal {
                            return Some(cmp);
                        }
                    }

                    list_self.len().partial_cmp(&list_other.len())
                }
            },
        };
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl FromStr for Node {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut list_stack: Vec<Vec<Node>> = vec![Vec::new()]; // implicit top-level list

        for token in TokenIterator::new(s) {
            match token {
                Invalid(pos) => return Err(anyhow!("invalid token at: {pos}")),
                Comma => continue,
                BrOpen => list_stack.push(Vec::new()),
                BrClose => {
                    let list = list_stack.pop().ok_or(anyhow!("unexpected end of list"))?;
                    let top = list_stack
                        .last_mut()
                        .ok_or(anyhow!("unexpected end of list"))?;
                    top.push(List(list));
                }
                Num(n) => {
                    let top = list_stack.last_mut().unwrap();
                    top.push(Int(n))
                }
            }
        }

        if list_stack.len() > 1 {
            Err(anyhow!("{} ']' tokens expected", list_stack.len() - 1))
        } else {
            Ok(list_stack.pop().unwrap().pop().unwrap())
        }
    }
}

enum Token {
    BrOpen,
    BrClose,
    Comma,
    Num(i32),
    Invalid(usize),
}

struct TokenIterator<'a> {
    s: &'a str,
    pos: usize,
}

// Yeah, it's primitive but it works

impl<'a> Iterator for TokenIterator<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let current_char = self.s.as_bytes().get(self.pos);

        match current_char {
            None => None,
            Some(char) => {
                self.pos += 1;

                Some(match char {
                    b',' => Comma,
                    b'[' => BrOpen,
                    b']' => BrClose,
                    b'0'..=b'9' => {
                        let mut result: i32 = (char - b'0') as i32;

                        while let Some(char) = self.s.as_bytes().get(self.pos) {
                            if !char.is_ascii_digit() {
                                break;
                            }
                            result = result * 10 + ((char - b'0') as i32);

                            self.pos += 1;
                        }

                        Num(result)
                    }
                    _ => Invalid(self.pos),
                })
            }
        }
    }
}

impl<'a> TokenIterator<'a> {
    fn new(s: &str) -> TokenIterator {
        TokenIterator { s, pos: 0 }
    }
}
