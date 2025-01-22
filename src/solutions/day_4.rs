#[derive(Copy, Clone)]
enum State {
    Start, X, M1, A1, S, A2, M2,
}

impl State {
    fn next(&mut self, c: char) -> bool {
        use State::*;
        match (*self, c) {
            (M2, 'X')    => { *self = X;  true },
            (A1, 'S')    => { *self = S;  true },
            (_, 'X') => { *self = X;  false},
            (_, 'S') => { *self = S;  false },
            (X, 'M')     => { *self = M1; false },
            (M1, 'A')    => { *self = A1; false },
            (S, 'A')     => { *self = A2; false },
            (A2, 'M')    => { *self = M2; false },
            _            => { *self = Start; false },
        }
    }
}

fn get_dimensions(input: &str) -> (usize, usize) {
    let length = input.len();
    let width = input.find('\n').expect("No newline found!");
    /*
       length = (width + 1)*height - 1
                         ^           ^
                      newline    no newline for last line
    */
    assert_eq!((length + 1) % (width + 1), 0, "Oops, can't find dimensions of input");
    let height = (length + 1)/(width + 1);
    (width, height)
}

pub fn clever(input: &str) -> u32 {
    let (width, height) = get_dimensions(input);
    let n_diagonals = width + height - 1;

    let mut states = vec![State::Start; width + 2*n_diagonals];
    let mut horizontal_state = State::Start;
    let (vertical_states, diagonal_states) = states.split_at_mut(width);
    let (up_diagonal_states, down_diagonal_states) = diagonal_states.split_at_mut(n_diagonals);
    
    let mut i = 0;
    let mut j = 0;
    let mut answer = 0;

    for c in input.chars() {
        if c == '\n' {
            i = 0;
            j += 1;
            horizontal_state = State::Start;
        } else {
            if horizontal_state.next(c) {
                answer += 1;
            }
            if vertical_states[i].next(c) {
                answer += 1;
            }
            if up_diagonal_states[i + height - j - 1].next(c) {
                answer += 1;
            }
            if down_diagonal_states[i + j].next(c) {
                answer += 1;
            }
            i += 1;
        }
    }

    answer
}

/* Credits to Tirthankar: https://github.com/wermos/advent-of-code/blob/main/2024/src/day4_part1.rs#L3 */
pub fn naive(input: &str) -> u32 {
    let input: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut hits = 0;

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == 'X' {
                if j < input[i].len() - 3 {
                    // check forwards
                    if input[i][j + 1] == 'M' && input[i][j + 2] == 'A' && input[i][j + 3] == 'S' {
                        hits += 1;
                    }
                }

                if j >= 3 {
                    // check backwards
                    if input[i][j - 3] == 'S' && input[i][j - 2] == 'A' && input[i][j - 1] == 'M' {
                        hits += 1;
                    }
                }

                if i >= 3 {
                    // check vertical
                    if input[i - 1][j] == 'M' && input[i - 2][j] == 'A' && input[i - 3][j] == 'S' {
                        hits += 1;
                    }

                    if j >= 3 {
                        // check backwards (upward) diagonal
                        if input[i - 1][j - 1] == 'M' && input[i - 2][j - 2] == 'A' && input[i - 3][j - 3] == 'S' {
                            hits += 1;
                        }
                    }

                    if j < input[i].len() - 3 {
                        // check forward (upward) diagonal
                        if input[i - 1][j + 1] == 'M' && input[i - 2][j + 2] == 'A' && input[i - 3][j + 3] == 'S' {
                            hits += 1;
                        }
                    }
                }

                if i < input.len() - 3 {
                    // check vertical
                    if input[i + 1][j] == 'M' && input[i + 2][j] == 'A' && input[i + 3][j] == 'S' {
                        hits += 1;
                    }

                    if j >= 3 {
                        // check backwards (downward) diagonal
                        if input[i + 1][j - 1] == 'M' && input[i + 2][j - 2] == 'A' && input[i + 3][j - 3] == 'S' {
                            hits += 1;
                        }
                    }

                    if j < input[i].len() - 3 {
                        // check forward (downwards) diagonal
                        if input[i + 1][j + 1] == 'M' && input[i + 2][j + 2] == 'A' && input[i + 3][j + 3] == 'S' {
                            hits += 1;
                        }
                    }
                }
            }
        }
    }
    
    hits
}