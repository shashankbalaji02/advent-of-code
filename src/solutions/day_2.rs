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
            let mut iter = line
                .split(' ')
                .map(|number| number.parse::<u32>().expect("Failed to parse number"));
            match iter.next() {
                Some(mut first) => match iter.next() {
                    Some(mut second) => {
                        let increasing = second > first;
                        loop {
                            let diff = second as i32 - first as i32;
                            if increasing && 1 <= diff && diff <= 3
                                || !increasing && -3 <= diff && diff <= -1
                            {
                                match iter.next() {
                                    Some(next) => {
                                        first = second;
                                        second = next;
                                    }
                                    None => {
                                        return 1;
                                    }
                                }
                                continue;
                            }
                            return 0;
                        }
                    }
                    None => 1,
                },
                None => 1,
            }
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