use crate::utils::{Matrix, read_lines};

enum OperationType {
    Add,
    Multiply,
}

struct Operation {
    operation_type: OperationType,
    tally: u64,
}

impl Operation {
    fn new(b: u8) -> Self {
        match b {
            b'+' => Self { operation_type: OperationType::Add, tally: 0 },
            b'*' => Self { operation_type: OperationType::Multiply, tally: 1 },
            _ => unreachable!(),
        }
    }

    fn apply(&mut self, value: u64) {
        match self.operation_type {
            OperationType::Add => self.tally += value,
            OperationType::Multiply => self.tally *= value,
        }
    }
}

pub fn puzzle1() {
    let mut lines: Vec<_> = read_lines(6).collect();
    let mut results: Vec<_> =
        lines.pop().unwrap().split_whitespace().map(|s| Operation::new(s.as_bytes()[0])).collect();
    for l in lines {
        for (i, n) in l.split_whitespace().enumerate() {
            results[i].apply(n.parse().unwrap());
        }
    }
    let result: u64 = results.into_iter().map(|o| o.tally).sum();
    println!("{result}");
}

pub fn puzzle2() {
    let mut lines: Vec<String> = read_lines(6).collect();
    let max_len = lines.iter().map(String::len).max().unwrap();
    for l in &mut lines {
        while l.len() < max_len {
            l.push(' ');
        }
    }
    let data = Matrix::from_row_iter(lines.into_iter().map(String::into_bytes)).unwrap();
    let y = data.rows() - 1;
    let mut x = 0;
    let mut result = 0;
    while let Some(mut operation) = data.get(x, y).copied().map(Operation::new) {
        let start = x;
        x += 1;
        let end = loop {
            match data.get(x, y) {
                Some(b' ') => x += 1,
                Some(b'+' | b'*') => break x - 1,
                None => break x,
                _ => unreachable!(),
            }
        };
        for x in start..end {
            let mut value = 0u64;
            for y in 0..y {
                let digit = match data[(x, y)] {
                    d @ b'0'..=b'9' => u64::from(d - b'0'),
                    b' ' => continue,
                    _ => unreachable!(),
                };
                value = value * 10 + digit;
            }
            operation.apply(value);
        }
        result += operation.tally;
    }
    println!("{result}");
}
