use std::iter::once;

pub fn parse_input(input: &String) -> Vec<Vec<u32>> {
    let mut result = Vec::new();
    input
        .lines()
        .for_each(|line| {
            let v = line
                .split(' ')
                .map(|number| number.parse().expect("Failed to parse number"))
                .collect();
            result.push(v);
        });
    result
}

pub fn task(input: &Vec<Vec<u32>>) -> u32 { 
    let mut safe_records = 0;
    'outer: for line in input.iter() {
        if line.len() < 2 {
            safe_records += 1;
            continue;
        }
        let increasing = line[1] > line[0];
        for i in 1..line.len() {
            let diff = line[i] as i32 - line[i - 1] as i32;
            if increasing && 1 <= diff && diff <= 3 || !increasing && -3 <= diff && diff <= -1 {
                continue;
            }
            continue 'outer;
        }
        safe_records += 1;
    }
    safe_records
}

pub fn combined(input: &String) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut nums = line
                .split(' ')
                .map(|number| number.parse::<u32>().expect("Failed to parse number"));
            
            let first = match nums.next() {
                Some(n) => n,
                None => return 1,
            };
            
            let second = match nums.next() {
                Some(n) => n,
                None => return 1,
            };
            
            let increasing = second > first;
            let mut prev_pair = (first, second);
            
            for next in nums.chain(once(0)) {
                let diff = prev_pair.1 as i32 - prev_pair.0 as i32;
                if increasing && 1 <= diff && diff <= 3 ||
                  !increasing && -3 <= diff && diff <= -1 {
                    prev_pair = (prev_pair.1, next);
                } else {
                    return 0;
                }
            }

            1
        })
        .sum()
}

pub fn split_loop_naive(input: &Vec<Vec<u32>>) -> u32 {
    let mut safe_records = 0;
    'outer: for line in input.iter() {
        if line.len() < 2 {
            safe_records += 1;
            continue;
        }
        let increasing = line[1] > line[0];
        if increasing {
            for i in 1..line.len() {
                let diff = line[i] as i32 - line[i - 1] as i32;
                if 1 <= diff && diff <= 3 {
                    continue;
                }
                continue 'outer;
            }
        } else {
            for i in 1..line.len() {
                let diff = line[i] as i32 - line[i - 1] as i32;
                if -3 <= diff && diff <= -1 {
                    continue;
                }
                continue 'outer;
            }
        }
        safe_records += 1;
    }
    safe_records
}

pub fn evaluate_string(input: &str) -> u32 {
    let mut current: u32 = 0;
    let mut prev: u32 = 0;
    let mut i: u32 = 0;
    let mut new_number = true;
    let mut increasing = false;
    let mut skip_line = false;
    let mut safe_records = 0;

    for c in input.chars().chain(once('\n')) {
        if skip_line && c != '\n' {
            continue;
        }
        match c {
            /* parse numbers */
            '0'..='9' => {
                if new_number {
                    prev = current;
                    current = c as u32 - '0' as u32;
                    new_number = false;
                } else {
                    current = 10*current + (c as u32 - '0' as u32);
                }
            },
            /* core logic: compare current and previous numbers */
            ' ' => {
                i += 1;
                new_number = true;
                if i == 2 {
                    increasing = current > prev;
                }
                if i >= 2 {
                    let diff = current as i32 - prev as i32;
                    if !(increasing && 1 <= diff && diff <= 3 ||
                        !increasing && -3 <= diff && diff <= -1) {
                        skip_line = true;
                    }
                }
            },
            /* take care of the last pair and state initialization for the next iteration */
            '\n' => {
                if !skip_line {
                    if i <= 1 {
                        safe_records += 1;
                    }
                    let diff = current as i32 - prev as i32;
                    if increasing && 1 <= diff && diff <= 3 ||
                      !increasing && -3 <= diff && diff <= -1 {
                        safe_records += 1;
                    }
                }
                current = 0;
                prev = 0;
                i = 0;
                new_number = true;
                increasing = false;
                skip_line = false;
            },
            _ => {
                unreachable!("Character not recognized");
            }
        }
    }
    safe_records
}