use criterion::{black_box, criterion_group, criterion_main, Criterion};

use advent_of_code_2022::solutions::{get_solver_for_day, INPUTS, N_DAYS};
use advent_of_code_2022::Stage;

fn bench_total(c: &mut Criterion) {
    c.bench_function("Full Advent", |b| {
        b.iter(|| {
            for d in 1..=N_DAYS {
                black_box(compute_answer(d, Stage::Easy));
                black_box(compute_answer(d, Stage::Hard));
            }
        })
    });
}

fn bench_easy(c: &mut Criterion) {
    let mut g = c.benchmark_group("Easy");

    for i in 1..=N_DAYS {
        let id = format!("Day{:02}", i);
        g.bench_function(&*id, |b| {
            b.iter(|| {
                black_box(compute_answer(i, Stage::Easy));
            });
        });
    }
}

fn bench_hard(c: &mut Criterion) {
    let mut g = c.benchmark_group("Hard");

    for i in 1..=N_DAYS {
        let id = format!("Day{:02}", i);
        g.bench_function(&*id, |b| {
            b.iter(|| {
                black_box(compute_answer(i, Stage::Hard));
            });
        });
    }
}

criterion_group!(benches, bench_total, bench_easy, bench_hard);
criterion_main!(benches);

fn compute_answer(day: u8, stage: Stage) -> String {
    get_solver_for_day(day)(stage, INPUTS[(day - 1) as usize])
}
