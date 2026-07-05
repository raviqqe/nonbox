#![allow(missing_docs)]

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use nonbox::{
    f62::{self, Float62},
    f64,
};

const ITERATION_COUNT: usize = 10000;

fn map<I, T>(
    criterion: &mut Criterion,
    name: &str,
    input: impl Fn(usize) -> I,
    function: impl Fn(I) -> T,
) {
    criterion.bench_function(name, |bencher| {
        bencher.iter(|| {
            for index in 0..ITERATION_COUNT {
                black_box(function(black_box(input(index))));
            }
        })
    });
}

fn binary<T>(
    criterion: &mut Criterion,
    name: &str,
    left: &[Float62],
    right: &[Float62],
    operate: impl Fn(Float62, Float62) -> T,
) {
    criterion.bench_function(name, |bencher| {
        bencher.iter(|| {
            for (x, y) in left.iter().zip(right) {
                black_box(operate(black_box(*x), black_box(*y)));
            }
        })
    });
}

fn unary<T>(
    criterion: &mut Criterion,
    name: &str,
    values: &[Float62],
    operate: impl Fn(Float62) -> T,
) {
    criterion.bench_function(name, |bencher| {
        bencher.iter(|| {
            for x in values {
                black_box(operate(black_box(*x)));
            }
        })
    });
}

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
    map(
        criterion,
        "f64_box_unsigned",
        |index| index as u64,
        f64::box_unsigned,
    );
    map(
        criterion,
        "f64_unbox_unsigned",
        |index| index as u64,
        f64::unbox_unsigned,
    );
    map(
        criterion,
        "f64_box_signed",
        |index| index as i64,
        f64::box_signed,
    );
    map(
        criterion,
        "f64_unbox_signed",
        |index| index as u64,
        f64::unbox_signed,
    );
    map(
        criterion,
        "f64_is_boxed",
        |index| index as u64,
        f64::is_boxed,
    );
}

fn f62(criterion: &mut Criterion) {
    map(
        criterion,
        "f62_box_payload",
        |index| index as u64,
        f62::box_payload,
    );
    map(
        criterion,
        "f62_unbox_payload",
        |index| index as u64,
        f62::unbox_payload,
    );
    map(
        criterion,
        "f62_unbox_payload_unchecked",
        |index| index as u64,
        f62::unbox_payload_unchecked,
    );
    map(
        criterion,
        "f62_box_integer",
        |index| index as i64,
        f62::box_integer,
    );
    map(
        criterion,
        "f62_unbox_integer",
        |index| index as u64,
        f62::unbox_integer,
    );
    map(
        criterion,
        "f62_unbox_integer_unchecked",
        |index| index as u64,
        f62::unbox_integer_unchecked,
    );
    map(
        criterion,
        "f62_box_float",
        |index| f64::from_bits(index as u64),
        f62::box_float,
    );
    criterion.bench_function("f62_box_float_special", |bencher| {
        let xs = (0..ITERATION_COUNT)
            .map(|x| [f64::INFINITY, f64::NEG_INFINITY, f64::NAN, 4.2][x % 4])
            .collect::<Vec<_>>();

        bencher.iter(|| {
            for x in &xs {
                black_box(f62::box_float(black_box(*x)));
            }
        })
    });
    map(
        criterion,
        "f62_unbox_float",
        |index| index as u64,
        f62::unbox_float,
    );
    map(
        criterion,
        "f62_unbox_float_unchecked",
        |index| index as u64,
        f62::unbox_float_unchecked,
    );
    map(
        criterion,
        "f62_is_integer",
        |index| index as u64,
        f62::is_integer,
    );
    map(
        criterion,
        "f62_is_payload",
        |index| index as u64,
        f62::is_payload,
    );
    map(
        criterion,
        "f62_is_float",
        |index| index as u64,
        f62::is_float,
    );

    let integers = (0..ITERATION_COUNT as i64)
        .map(Float62::from_integer)
        .collect::<Vec<_>>();
    let descending_integers = (0..ITERATION_COUNT as i64)
        .map(|x| Float62::from_integer(ITERATION_COUNT as i64 - x))
        .collect::<Vec<_>>();
    let divisor_integers = (0..ITERATION_COUNT as i64)
        .map(|x| Float62::from_integer(x % 256 + 1))
        .collect::<Vec<_>>();
    let overflowing_integers = (0..ITERATION_COUNT as i64)
        .map(|x| Float62::from_integer((1 << 61) + x % 256))
        .collect::<Vec<_>>();
    let large_integers = (0..ITERATION_COUNT as i64)
        .map(|x| Float62::from_integer((1 << 40) + x))
        .collect::<Vec<_>>();
    let floats = (0..ITERATION_COUNT)
        .map(|x| Float62::from_float(x as f64 + 0.5))
        .collect::<Vec<_>>();
    let descending_floats = (0..ITERATION_COUNT)
        .map(|x| Float62::from_float((ITERATION_COUNT - x) as f64 + 0.5))
        .collect::<Vec<_>>();
    let divisor_floats = (0..ITERATION_COUNT)
        .map(|x| Float62::from_float((x % 256) as f64 + 1.5))
        .collect::<Vec<_>>();

    binary(
        criterion,
        "f62_add_integer",
        &integers,
        &integers,
        |x, y| x + y,
    );
    binary(
        criterion,
        "f62_sub_integer",
        &integers,
        &integers,
        |x, y| x - y,
    );
    binary(
        criterion,
        "f62_mul_integer",
        &integers,
        &integers,
        |x, y| x * y,
    );
    binary(
        criterion,
        "f62_div_integer",
        &integers,
        &divisor_integers,
        |x, y| x / y,
    );
    binary(
        criterion,
        "f62_rem_integer",
        &integers,
        &divisor_integers,
        |x, y| x % y,
    );
    binary(
        criterion,
        "f62_checked_rem_integer",
        &integers,
        &divisor_integers,
        |x, y| x.checked_rem(y),
    );
    unary(criterion, "f62_neg_integer", &integers, |x| -x);

    binary(criterion, "f62_add_float", &floats, &floats, |x, y| x + y);
    binary(criterion, "f62_sub_float", &floats, &floats, |x, y| x - y);
    binary(criterion, "f62_mul_float", &floats, &floats, |x, y| x * y);
    binary(
        criterion,
        "f62_div_float",
        &floats,
        &divisor_floats,
        |x, y| x / y,
    );
    binary(
        criterion,
        "f62_rem_float",
        &floats,
        &divisor_floats,
        |x, y| x % y,
    );
    unary(criterion, "f62_neg_float", &floats, |x| -x);

    binary(criterion, "f62_add_mixed", &integers, &floats, |x, y| x + y);
    binary(criterion, "f62_mul_mixed", &integers, &floats, |x, y| x * y);

    binary(
        criterion,
        "f62_add_integer_overflow",
        &overflowing_integers,
        &overflowing_integers,
        |x, y| x + y,
    );
    binary(
        criterion,
        "f62_mul_integer_overflow",
        &large_integers,
        &large_integers,
        |x, y| x * y,
    );

    binary(
        criterion,
        "f62_cmp_integer",
        &integers,
        &descending_integers,
        |x, y| x.partial_cmp(&y),
    );
    binary(
        criterion,
        "f62_cmp_float",
        &floats,
        &descending_floats,
        |x, y| x.partial_cmp(&y),
    );
    binary(criterion, "f62_cmp_mixed", &integers, &floats, |x, y| {
        x.partial_cmp(&y)
    });
}

criterion_group!(benches, sum, f64, f62);

criterion_main!(benches);
