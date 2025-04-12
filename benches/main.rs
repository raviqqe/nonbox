#![allow(missing_docs)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nonbox::f64;

const ITERATION_COUNT: usize = 10000;

fn sum(criterion: &mut Criterion) {
    criterion.bench_function("sum", |bencher| {
        bencher.iter(|| {
            let mut sum = 0;

            for index in 0..ITERATION_COUNT as u64 {
                sum += black_box(index);
            }

            black_box(sum);
        })
    });
}

fn box_unsigned(criterion: &mut Criterion) {
    criterion.bench_function("box_unsigned", |bencher| {
        bencher.iter(|| {
            for index in 0..ITERATION_COUNT as u64 {
                black_box(f64::box_unsigned(index));
            }
        })
    });
}

fn box_signed(criterion: &mut Criterion) {
    criterion.bench_function("box_signed", |bencher| {
        bencher.iter(|| {
            for index in 0..ITERATION_COUNT as i64 {
                black_box(f64::box_signed(index));
            }
        })
    });
}

criterion_group!(benches, sum, box_unsigned, box_signed);

criterion_main!(benches);
