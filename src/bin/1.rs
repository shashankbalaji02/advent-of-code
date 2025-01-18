#![feature(test)]
extern crate test;

use std::iter::zip;
use std::fs::read_to_string;

/* https://adventofcode.com/2024/day/1 */
fn task(a: &mut Vec<u32>, b: &mut Vec<u32>) -> u64 {
    a.sort();
    b.sort();
    assert_eq!(a.len(), b.len(), "Columns are of unequal length");
    iter_algo(&a, &b)
}

fn iter_algo(a: &Vec<u32>, b: &Vec<u32>) -> u64 {
    zip(a.iter(), b.iter())
        .map(|pair| (*pair.0 as i64 - *pair.1 as i64).abs() as u64)
        .sum()
}

fn read_input(filename: &str) -> (Vec<u32>, Vec<u32>) {
    let mut a: Vec<u32> = Vec::new();
    let mut b: Vec<u32> = Vec::new();
    read_to_string(filename)
        .expect("Can't read input to string")
        .lines()
        .for_each(|line| {
            let pair = line.split_once(" ").expect("Line missing space");
            a.push(pair.0.parse().expect("Can't parse first number"));
            b.push(pair.1.trim().parse().expect("Can't parse second number"));
        });
    (a, b)
}

const INPUT_FILENAME: &str = "inputs/1.txt";

fn main() {
    let (mut a, mut b) = read_input(INPUT_FILENAME);
    println!("{}", task(&mut a, &mut b));                                    
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    fn for_loop_algo(a: &Vec<u32>, b: &Vec<u32>) -> u64 {
        let mut sum = 0;
        for i in 0..a.len() {
            sum += (a[i] as i64 - b[i] as i64).abs() as u64
        }
        sum
    }
    
    #[bench]
    fn iter_algo_bench(bencher: &mut Bencher) {
        let (mut a, mut b) = read_input(INPUT_FILENAME);
        a.sort();
        b.sort();
        assert_eq!(a.len(), b.len(), "Columns are of unequal length");
        bencher.iter(|| iter_algo(&a, &b));
    }
    
    #[bench]
    fn for_loop_algo_bench(bencher: &mut Bencher) {
        let (mut a, mut b) = read_input(INPUT_FILENAME);
        a.sort();
        b.sort();
        assert_eq!(a.len(), b.len(), "Columns are of unequal length");
        bencher.iter(|| for_loop_algo(&a, &b));
    }
}