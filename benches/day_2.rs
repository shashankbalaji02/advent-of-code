use std::fs::read_to_string;
use advent_of_code::solutions::day_2::*;
use criterion::{criterion_group, criterion_main, Criterion};

const INPUT_FILENAME: &str = concat!(env!("INPUT_DIR"), "/day_2.txt");

fn bench(c: &mut Criterion) {
    let input_string = read_to_string(INPUT_FILENAME).expect("Failed to read input");
    let input = parse_input(&input_string);

    let mut group = c.benchmark_group("combined vs. naive vs. string evaluation");
    group.bench_function("combined", |bencher| bencher.iter(|| combined(&input_string)));
    group.bench_function("naive", |bencher| bencher.iter(|| task(&parse_input(&input_string))));
    group.bench_function("string evaluation", |bencher| bencher.iter(|| evaluate_string(&input_string)));
    group.finish();

    group = c.benchmark_group("(naive) split loop vs. combined loop");
    group.bench_function("split loop", |b| b.iter(|| split_loop_naive(&input)));
    group.bench_function("combined loop", |b| b.iter(|| task(&input)));
    group.finish()
}

criterion_group!(benches, bench);
criterion_main!(benches);