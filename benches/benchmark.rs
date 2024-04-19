use criterion::{black_box, Criterion, criterion_group, criterion_main};

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

    for i in 1..=N_DAYS {
        for stage in [Stage::Easy, Stage::Hard] {
            let id = format!("Day {:02} - {}", i, stage);
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

fn compute_answer(day: u8, stage: Stage) -> String {
    get_solver_for_day(day)(stage, INPUTS[(day - 1) as usize])
}
