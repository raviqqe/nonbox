#![allow(missing_docs)]

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use nonbox::{
    f62::{self, Float62},
    f64,
};

const ITERATION_COUNT: usize = 10000;

fn sum(criterion: &mut Criterion) {
    criterion.bench_function("sum_u64", |bencher| {
        let xs = (0..ITERATION_COUNT as i64).collect::<Vec<_>>();

        bencher.iter(|| {
            let mut sum = 0;

            for x in &xs {
                sum += black_box(*x);
            }

            black_box(sum);
        })
    });

    criterion.bench_function("sum_f64", |bencher| {
        let xs = (0..ITERATION_COUNT as u64)
            .map(f64::from_bits)
            .collect::<Vec<_>>();

        bencher.iter(|| {
            let mut sum = 0.0;

            for x in &xs {
                sum += black_box(*x);
            }

            black_box(sum);
        })
    });

    criterion.bench_function("sum_f62", |bencher| {
        let xs = (0..ITERATION_COUNT as i64)
            .map(Float62::from_integer)
            .collect::<Vec<_>>();

        bencher.iter(|| {
            let mut sum = Float62::default();

            for x in &xs {
                sum += black_box(*x);
            }

            black_box(sum);
        })
    });
}

fn f64(criterion: &mut Criterion) {
    criterion.bench_function("f64_box_unsigned", |bencher| {
        bencher.iter(|| {
            for index in 0..ITERATION_COUNT as u64 {
                black_box(f64::box_unsigned(black_box(index)));
            }
        })
    });

    criterion.bench_function("f64_unbox_unsigned", |bencher| {
        bencher.iter(|| {
            for index in 0..ITERATION_COUNT as u64 {
                black_box(f64::unbox_unsigned(black_box(index)));
            }
        })
    });

    criterion.bench_function("f64_box_signed", |bencher| {
        bencher.iter(|| {
            for index in 0..ITERATION_COUNT as i64 {
                black_box(f64::box_signed(black_box(index)));
            }
        })
    });

    criterion.bench_function("f64_unbox_signed", |bencher| {
        bencher.iter(|| {
            for index in 0..ITERATION_COUNT as u64 {
                black_box(f64::unbox_signed(black_box(index)));
            }
        })
    });
}

fn f62(criterion: &mut Criterion) {
    criterion.bench_function("f62_box_payload", |bencher| {
        bencher.iter(|| {
            for index in 0..ITERATION_COUNT as u64 {
                black_box(f62::box_payload(black_box(index)));
            }
        })
    });

    criterion.bench_function("f62_unbox_payload", |bencher| {
        bencher.iter(|| {
            for index in 0..ITERATION_COUNT as u64 {
                black_box(f62::unbox_payload(black_box(index)));
            }
        })
    });

    criterion.bench_function("f62_box_integer", |bencher| {
        bencher.iter(|| {
            for index in 0..ITERATION_COUNT as i64 {
                black_box(f62::box_integer(black_box(index)));
            }
        })
    });

    criterion.bench_function("f62_unbox_integer", |bencher| {
        bencher.iter(|| {
            for index in 0..ITERATION_COUNT as u64 {
                black_box(f62::unbox_integer(black_box(index)));
            }
        })
    });

    criterion.bench_function("f62_box_float", |bencher| {
        bencher.iter(|| {
            for index in 0..ITERATION_COUNT as u64 {
                black_box(f62::box_float(black_box(f64::from_bits(index))));
            }
        })
    });

    criterion.bench_function("f62_unbox_float", |bencher| {
        bencher.iter(|| {
            for index in 0..ITERATION_COUNT as u64 {
                black_box(f62::unbox_float(black_box(index)));
            }
        })
    });
}

criterion_group!(benches, sum, f64, f62);

criterion_main!(benches);
