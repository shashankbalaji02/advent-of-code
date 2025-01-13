use std::iter::zip;
use std::fs::read_to_string;

/* https://adventofcode.com/2024/day/1 */
fn task(a: &mut Vec<u32>, b: &mut Vec<u32>) -> u64 {
    a.sort();
    b.sort();
    assert_eq!(a.len(), b.len(), "Columns are of unequal length");
    zip(a.iter(), b.iter())
        .map(|pair| (*pair.0 as i64 - *pair.1 as i64).abs() as u64)
        .sum()
}

fn main() {
    let mut a: Vec<u32> = Vec::new();
    let mut b: Vec<u32> = Vec::new();
    read_to_string("input.txt").expect("Can't read input to string")
                                     .lines()
                                     .for_each(|line| {
                                        let pair = line.split_once(" ").expect("Line missing space");
                                        a.push(pair.0.parse().expect("Can't parse first number"));
                                        b.push(pair.1.trim().parse().expect("Can't parse second number"));
                                     });
    println!("{}", task(&mut a, &mut b));                                    
}