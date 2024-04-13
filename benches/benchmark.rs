use std::collections::HashMap;

use criterion::{black_box, Criterion, criterion_group, criterion_main};
use lazy_static::lazy_static;

use advent_of_code_2022::solutions::{get_solver_for_day, N_DAYS};
use advent_of_code_2022::{process_input, Stage};

fn bench_total(c: &mut Criterion) {
    c.bench_function("Full Advent", |b| {
        b.iter(|| {
            for d in 1..=N_DAYS {
                black_box(compute_answer(d, Stage::Easy));
                black_box(compute_answer(d, Stage::Hard));
            }
        })
    });

    for i in 1..=N_DAYS {
        for stage in [Stage::Easy, Stage::Hard] {
            let id = format!("Day {} - {}", i, stage);
            c.bench_function(&*id, |b| {
                b.iter(|| {
                    black_box(compute_answer(i, stage));
                });
            });
        }
    }
}

criterion_group!(benches, bench_total);
criterion_main!(benches);

lazy_static! {
    static ref INPUT_CACHE: HashMap<u8, Vec<&'static str>> = {
        let mut m = HashMap::new();

        for d in 1..=N_DAYS {
            m.insert(d, process_input(d));
        }
        m
    };
}

fn get_input(day: u8) -> &'static Vec<&'static str> {
    INPUT_CACHE.get(&day).unwrap()
}

fn compute_answer(day: u8, stage: Stage) -> String {
    get_solver_for_day(day)(stage, get_input(day))
}
