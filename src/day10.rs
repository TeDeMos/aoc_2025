use crate::utils::{Matrix, Rational, read_lines};

pub fn puzzle1() {
    let result: u32 = read_lines(10)
        .map(|l| {
            let mut iter = l.split(' ');
            let first = iter.next().unwrap().as_bytes();
            let target = first[1..first.len() - 1]
                .iter()
                .enumerate()
                .map(|(i, b)| match b {
                    b'.' => 0,
                    b'#' => 1u16 << i,
                    _ => unreachable!(),
                })
                .reduce(|a, n| a | n)
                .unwrap();
            let buttons: Vec<_> = iter
                .take_while(|s| s.starts_with('('))
                .map(|s| {
                    s.trim_start_matches('(')
                        .trim_end_matches(')')
                        .split(',')
                        .map(|n| 1u16 << n.parse::<u16>().unwrap())
                        .reduce(|a, n| a | n)
                        .unwrap()
                })
                .collect();
            find(0, target, &buttons, 0).unwrap()
        })
        .sum();
    println!("{result}");
}

// Clicking a button more than once does not do anything, brute force search for buttons to click
fn find(current: u16, target: u16, remaining: &[u16], clicked: u32) -> Option<u32> {
    if current == target {
        return Some(clicked);
    }
    if remaining.is_empty() {
        return None;
    }
    let a = find(current, target, &remaining[1..], clicked);
    let b = find(current ^ remaining[0], target, &remaining[1..], clicked + 1);
    match (a, b) {
        (Some(a), Some(b)) => Some(a.min(b)),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

// Based on https://www.reddit.com/r/adventofcode/comments/1pity70/comment/ntga1h2
pub fn puzzle2() {
    let result: i64 = read_lines(10)
        .enumerate()
        .map(|(i, l)| {
            let mut v: Vec<_> = l.split(' ').skip(1).collect();
            let mut max = 0;
            let mut sum = 0;
            let constants: Vec<_> = v
                .pop()
                .unwrap()
                .trim_start_matches('{')
                .trim_end_matches('}')
                .split(',')
                .map(|n| {
                    let n = n.parse().unwrap();
                    max = max.max(n);
                    sum += n;
                    n
                })
                .collect();
            let mut matrix: Matrix<Rational> =
                Matrix::new_default(v.len() + 1, constants.len() + 1);
            let columns = matrix.columns();
            let rows = matrix.rows();
            constants
                .into_iter()
                .enumerate()
                .for_each(|(y, n)| matrix[(columns - 1, y)] = n.into());
            v.into_iter().enumerate().for_each(|(x, b)| {
                b.trim_start_matches('(')
                    .trim_end_matches(')')
                    .split(',')
                    .map(|n| n.parse::<usize>().unwrap())
                    .for_each(|n| matrix[(x, n)] = Rational::ONE);
            });
            matrix.get_row_mut(rows - 1).unwrap()[..columns - 1].fill(Rational::ONE);
            (max..=sum)
                .find(|&n| {
                    matrix[(columns - 1, rows - 1)] = n.into();
                    check_matrix(&matrix, max)
                })
                .unwrap_or_else(|| panic!("{i}\n{matrix:?}"))
        })
        .sum();
    println!("{result}");
}

fn check_matrix(matrix: &Matrix<Rational>, limit: i64) -> bool {
    // solving equation
    let mut matrix = matrix.clone();
    let columns = matrix.columns();
    let rows = matrix.rows();
    let mut current_row = 0;
    for column in 0..columns - 1 {
        let Some(r) = (current_row..rows).find(|&r| matrix[(column, r)] != Rational::ZERO) else {
            continue;
        };
        if r != current_row {
            matrix.swap_rows(current_row, r);
        }
        matrix.scale_row(current_row, Rational::ONE / matrix[(column, current_row)]);
        for r in (0..rows).filter(|&r| r != current_row) {
            matrix.add_rows(r, current_row, -matrix[(column, r)]);
        }
        current_row += 1;
        if current_row == rows {
            break;
        }
    }

    // pivots and free variables
    let mut pivot = Vec::new();
    let mut free = Vec::new();
    for c in 0..columns - 1 {
        let mut ones = 0;
        let mut other = false;
        for r in 0..rows {
            if matrix[(c, r)] == Rational::ONE {
                ones += 1;
            } else if matrix[(c, r)] != Rational::ZERO {
                other = true;
            }
        }
        if ones == 1 && !other {
            pivot.push(c);
        } else if ones != 0 || other {
            free.push(c);
        }
    }

    // contradictions
    if matrix.iter_rows().any(|mut r| {
        *r.split_off_last().unwrap() != Rational::ZERO && r.iter().all(|&v| v == Rational::ZERO)
    }) {
        return false;
    }

    // negative and non-integer solutions, limiting the search for the last free variable
    let mut last_free_limit = Rational::from(limit);
    for p in pivot {
        let r = (0..rows).find(|&r| matrix[(p, r)] == 1.into()).unwrap();
        let mut just_pivot = true;
        let mut all_positive = true;
        for (i, &f) in free.iter().enumerate() {
            let value = matrix[(f, r)];
            if value != Rational::ZERO {
                // if no free variables before or each was positive, upper limit is now the constant
                if (just_pivot || all_positive) && i == free.len() - 1 && value > Rational::ZERO {
                    last_free_limit = last_free_limit.min(matrix[(columns - 1, r)] / value);
                }
                if value < Rational::ZERO {
                    all_positive = false;
                }
                just_pivot = false;
            }
        }
        if just_pivot {
            let value = matrix[(columns - 1, r)];
            if value < Rational::ZERO || !value.is_integer() {
                return false;
            }
        }
    }

    if let Some(&f) = free.last() {
        // Free variable search
        let mut extended = matrix.with_new_default_row();
        extended[(f, rows)] = Rational::ONE;
        (0..=last_free_limit.ceil()).any(|n| {
            extended[(columns - 1, rows)] = n.into();
            check_matrix(&extended, limit)
        })
    } else {
        true
    }
}
