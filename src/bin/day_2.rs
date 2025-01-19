use advent_of_code::solutions::day_2::*;
use std::fs::read_to_string;

const INPUT_FILENAME: &str = concat!(env!("INPUT_DIR"), "/day_2.txt");

fn main() {
    let input = parse_input(&read_to_string(INPUT_FILENAME).expect("Failed to read input"));
    println!("{}", task(&input));
}

#[test]
fn test_equivalence() {
    let input = read_to_string(INPUT_FILENAME).expect("Failed to read input");
    let answer = task(&parse_input(&input));
    assert_eq!(combined(&input), answer);
    assert_eq!(split_loop_naive(&parse_input(&input)), answer);
}