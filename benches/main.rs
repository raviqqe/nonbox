#![allow(missing_docs)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nonbox::f64;

const ITERATION_COUNT: usize = 1000;

fn bench(criterion: &mut Criterion) {
    criterion.bench_function("sum", |bencher| {
        bencher.iter(|| {
            let mut sum = 0.0f64;

            for index in 0..black_box(ITERATION_COUNT as u64) {
                sum = sum + f64::box_unsigned(index);
            }

            black_box(sum);
        })
    });
}

criterion_group!(benches, bench);

criterion_main!(benches);
