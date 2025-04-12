#![allow(missing_docs)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nonbox::{f64, n64::N64};

const ITERATION_COUNT: usize = 10000;

fn sum(criterion: &mut Criterion) {
    criterion.bench_function("sum_native", |bencher| {
        bencher.iter(|| {
            let mut sum = 0.0f64;

            for index in 0..black_box(ITERATION_COUNT as u64) {
                sum += index as f64;
            }

            black_box(sum);
        })
    });

    criterion.bench_function("sum_f64", |bencher| {
        bencher.iter(|| {
            let mut sum = 0.0f64;

            for index in 0..black_box(ITERATION_COUNT as u64) {
                sum += f64::from_bits(f64::box_unsigned(index));
            }

            black_box(sum);
        })
    });

    criterion.bench_function("sum_n64", |bencher| {
        bencher.iter(|| {
            let mut sum = N64::from_signed_integer(0);

            for index in 0..black_box(ITERATION_COUNT as u64) {
                sum = sum + N64::from_signed_integer(index as _);
            }

            black_box(sum);
        })
    });
}

criterion_group!(benches, sum);

criterion_main!(benches);
