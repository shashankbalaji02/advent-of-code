use std::fs::read_to_string;
use std::iter::zip;

/* https://adventofcode.com/2024/day/1 */
pub fn task(a: &Vec<u32>, b: &Vec<u32>) -> u64 {
    let mut a = a.clone();
    let mut b = b.clone();
    a.sort();
    b.sort();
    assert_eq!(a.len(), b.len(), "Columns are of unequal length");
    iter_algo(&a, &b)
}

pub fn iter_algo(a: &Vec<u32>, b: &Vec<u32>) -> u64 {
    zip(a.iter(), b.iter())
        .map(|pair| (*pair.0 as i64 - *pair.1 as i64).abs() as u64)
        .sum()
}

pub fn for_loop_algo(a: &Vec<u32>, b: &Vec<u32>) -> u64 {
    let mut sum = 0;
    for i in 0..a.len() {
        sum += (a[i] as i64 - b[i] as i64).abs() as u64
    }
    sum
}

pub fn parse_input(filename: &str) -> (Vec<u32>, Vec<u32>) {
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