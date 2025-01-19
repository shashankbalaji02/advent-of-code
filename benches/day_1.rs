use advent_of_code::solutions::day_1::*;
use criterion::{criterion_group, criterion_main, Criterion};

const INPUT_FILENAME: &str = concat!(env!("INPUT_DIR"), "/day_1.txt");

fn bench(c: &mut Criterion) {
    let (mut a, mut b) = parse_input(INPUT_FILENAME);
    assert_eq!(a.len(), b.len(), "Columns are of unequal length");
    a.sort();
    b.sort();
    let mut group = c.benchmark_group("iter vs. for loop");
    group.bench_function("iter", |bencher| bencher.iter(|| iter_algo(&a, &b)));
    group.bench_function("for loop", |bencher| bencher.iter(|| for_loop_algo(&a, &b)));
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);