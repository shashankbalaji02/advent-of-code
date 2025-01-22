use advent_of_code::solutions::day_4::*;
use std::fs::read_to_string;

use perf_event::{Builder, Group};
use perf_event::events::Hardware;

const INPUT_FILENAME: &str = concat!(env!("INPUT_DIR"), "/day_4.txt");

fn main() -> std::io::Result<()> {
    let input_string = read_to_string(INPUT_FILENAME).expect("Failed to read input");
    // println!("Part 1: {} {}", clever(&input_string), naive(&input_string));
    let mut group = Group::new()?;
    let cycles = Builder::new().group(&mut group).kind(Hardware::CPU_CYCLES).build()?;
    let insns = Builder::new().group(&mut group).kind(Hardware::INSTRUCTIONS).build()?;
    let cache_miss = Builder::new().group(&mut group).kind(Hardware::CACHE_MISSES).build()?;
    let cache_access = Builder::new().group(&mut group).kind(Hardware::CACHE_REFERENCES).build()?;
    group.enable()?;
    println!("{}", clever(&input_string));
    group.disable()?;
    let counts = group.read()?;
    println!("[clever] cycles: {}, instructions: {}, cache misses: {}, cache accesses: {}",
                counts[&cycles],
                counts[&insns],
                counts[&cache_miss],
                counts[&cache_access]);
    group.reset()?;
    group.enable()?;
    println!("{}", naive(&input_string));
    group.disable()?;

    let counts = group.read()?;
    println!("[naive] cycles: {}, instructions: {}, cache misses: {}, cache accesses: {}",
                counts[&cycles],
                counts[&insns],
                counts[&cache_miss],
                counts[&cache_access]);
    Ok(())
}