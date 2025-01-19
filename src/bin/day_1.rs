use advent_of_code::solutions::day_1::*;

const INPUT_FILENAME: &str = concat!(env!("INPUT_DIR"), "/day_1.txt");

fn main() {
    let (a, b) = parse_input(INPUT_FILENAME);
    println!("{}", task(&a, &b));                                    
}

#[test]
fn test_equivalence() {
    let (mut a, mut b) = parse_input(INPUT_FILENAME);
    a.sort();
    b.sort();
    assert_eq!(iter_algo(&a, &b), for_loop_algo(&a, &b));
}