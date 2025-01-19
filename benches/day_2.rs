#![feature(test)]
extern crate test;
use test::Bencher;
use advent_of_code::solutions::day_2::*;
use std::fs::read_to_string;

const INPUT_FILENAME: &str = concat!(env!("INPUT_DIR"), "/day_2.txt");

#[bench]
fn a_combined_bench(bencher: &mut Bencher) {
    let input = read_to_string(INPUT_FILENAME).expect("Failed to read input");
    bencher.iter(|| combined(&input))
}

#[bench]
fn a_naive_bench(bencher: &mut Bencher) {
    let input = read_to_string(INPUT_FILENAME).expect("Failed to read input");
    bencher.iter(|| task(&parse_input(&input)))
}

#[bench]
fn b_split_loop_naive_bench(bencher: &mut Bencher) {
    let input = parse_input(&read_to_string(INPUT_FILENAME).expect("Failed to read input"));
    bencher.iter(|| split_loop_naive(&input))
}

#[bench]
fn b_naive_bench(bencher: &mut Bencher) {
    let input = parse_input(&read_to_string(INPUT_FILENAME).expect("Failed to read input"));
    bencher.iter(|| task(&input))
}