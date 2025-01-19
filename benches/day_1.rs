#![feature(test)]
extern crate test;
use test::Bencher;
use advent_of_code::solutions::day_1::*;

const INPUT_FILENAME: &str = concat!(env!("INPUT_DIR"), "/day_1.txt");

#[bench]
fn iter_algo_bench(bencher: &mut Bencher) {
    let (mut a, mut b) = parse_input(INPUT_FILENAME);
    a.sort();
    b.sort();
    assert_eq!(a.len(), b.len(), "Columns are of unequal length");
    bencher.iter(|| iter_algo(&a, &b));
}

#[bench]
fn for_loop_algo_bench(bencher: &mut Bencher) {
    let (mut a, mut b) = parse_input(INPUT_FILENAME);
    a.sort();
    b.sort();
    assert_eq!(a.len(), b.len(), "Columns are of unequal length");
    bencher.iter(|| for_loop_algo(&a, &b));
}
