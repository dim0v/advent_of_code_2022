#![feature(test)]
extern crate test;

use seq_macro::seq;
use std::collections::HashMap;
use test::{black_box, Bencher};

use advent_of_code_2022::solutions::get_solver_for_day;
use advent_of_code_2022::*;
use lazy_static::lazy_static;

const N_DAYS: u8 = 7;

static ANSWERS: [[&str; 2]; N_DAYS as usize] = [
    ["69795", "208437"],
    ["13268", "15508"],
    ["7785", "2633"],
    ["569", "936"],
    ["RLFNRTNFB", "MHQTLJRLB"],
    ["1582", "3588"],
    ["1908462", "3979145"],
];

seq!(N in 1..=7 {
    #(
    #[test]
    fn easy_day~N() {
        assert_eq!(ANSWERS[N - 1][0], compute_answer(N, Stage::Easy));
    }

    #[test]
    fn hard_day~N() {
        assert_eq!(ANSWERS[N - 1][1], compute_answer(N, Stage::Hard));
    }

    #[bench]
    fn bench_easy_day~N(b: &mut Bencher) {
        b.iter(|| { black_box(compute_answer(N, Stage::Easy)); });
    }

    #[bench]
    fn bench_hard_day~N(b: &mut Bencher) {
        b.iter(|| { black_box(compute_answer(N, Stage::Hard)); });
    }
    )*
});

#[bench]
fn bench_total(b: &mut Bencher) {
    b.iter(|| {
        for d in 1..=N_DAYS {
            black_box(compute_answer(d, Stage::Easy));
            black_box(compute_answer(d, Stage::Hard));
        }
    });
}

lazy_static! {
    static ref INPUT_CACHE: HashMap<u8, Vec<String>> = {
        let mut m = HashMap::new();

        for d in 1..=N_DAYS {
            m.insert(d, read_input(d).unwrap());
        }
        m
    };
}

fn get_input(day: u8) -> &'static Vec<String> {
    INPUT_CACHE.get(&day).unwrap()
}

fn compute_answer(day: u8, stage: Stage) -> String {
    get_solver_for_day(day)(stage, get_input(day))
}
