use crate::Stage;
use std::collections::HashSet;
use std::hash::RandomState;

pub fn solve(stage: Stage, input: &str) -> String {
    let src = input.lines();

    let result: i64 = match stage {
        Stage::Easy => src
            .map(|x| split_in_halves(x))
            .map(|x| get_common_char(x.iter().map(|x| *x)))
            .map(|c| get_priority(c))
            .sum(),
        Stage::Hard => src
            .collect::<Vec<&str>>()
            .chunks(3)
            .map(|x| get_common_char(x.iter().map(|x| *x)))
            .map(|c| get_priority(c))
            .sum(),
    };

    result.to_string()
}

fn split_in_halves(row: &str) -> [&str; 2] {
    let halfway = row.len() / 2;

    [&row[..halfway], &row[halfway..]]
}

fn get_common_char<'a>(strings: impl Iterator<Item = &'a str>) -> char {
    let intersection = strings
        .map(|s| HashSet::from_iter(s.chars()))
        .reduce(|cur: HashSet<char, RandomState>, x| cur.intersection(&x).map(|x| *x).collect())
        .unwrap();

    intersection.iter().next().unwrap().clone()
}

fn get_priority(c: char) -> i64 {
    match c {
        'a'..='z' => c as i64 - 'a' as i64 + 1,
        'A'..='Z' => c as i64 - 'A' as i64 + 27,
        _ => {
            panic!()
        }
    }
}
