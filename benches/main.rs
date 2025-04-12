#![allow(missing_docs)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nonbox::f64;

const ITERATION_COUNT: usize = 10000;

fn sum(criterion: &mut Criterion) {
    criterion.bench_function("sum_pure", |bencher| {
        bencher.iter(|| {
            let mut sum = 0.0f64;

            for index in 0..black_box(ITERATION_COUNT as u64) {
                sum += index as f64;
            }

            black_box(sum);
        })
    });
}

fn box_unsigned(criterion: &mut Criterion) {
    criterion.bench_function("box_unsigned", |bencher| {
        bencher.iter(|| {
            let mut sum = 0.0f64;

            for index in 0..black_box(ITERATION_COUNT as u64) {
                sum += f64::from_bits(f64::box_unsigned(index));
            }

            black_box(sum);
        })
    });
}

fn box_signed(criterion: &mut Criterion) {
    criterion.bench_function("box_signed", |bencher| {
        bencher.iter(|| {
            let mut sum = 0.0f64;

            for index in 0..black_box(ITERATION_COUNT as i64) {
                sum += f64::from_bits(f64::box_signed(index));
            }

            black_box(sum);
        })
    });
}

criterion_group!(benches, sum, box_unsigned, box_signed);

criterion_main!(benches);
