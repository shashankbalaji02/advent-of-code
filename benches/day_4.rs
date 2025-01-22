use std::fs::read_to_string;
use advent_of_code::solutions::day_4::*;
use criterion::{criterion_group, criterion_main, Criterion};

const INPUT_FILENAME: &str = concat!(env!("INPUT_DIR"), "/day_4.txt");

fn bench(c: &mut Criterion) {
    let input_string = read_to_string(INPUT_FILENAME).expect("Failed to read input");

    let mut group = c.benchmark_group("clever vs. naive");
    group.bench_function("clever", |bencher| bencher.iter(|| clever(&input_string)));
    group.bench_function("naive", |bencher| bencher.iter(|| naive(&input_string)));
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);