use crate::utils::read_lines;

pub fn puzzle1() {
    let mut result = 0;
    for l in read_lines(3) {
        let batteries = l.chars().map(|c| c.to_digit(10).unwrap());
        let mut max_first = 0;
        let mut max_second = 0;
        let mut potential = 0;
        for b in batteries {
            if potential > max_first {
                max_first = potential;
                max_second = b;
                potential = 0;
            } else if b > max_second {
                max_second = b;
            }
            if b > potential {
                potential = b;
            }
        }
        result += max_first * 10 + max_second;
    }
    println!("{result}");
}

fn biggest(mut chosen: [u8; 12], digits: &[u8], current: usize) -> u64 {
    for n in (1u8..=9).rev() {
        let Some(p) = digits.iter().position(|&d| d == n) else { continue };
        chosen[current] = n;
        if current == 11 {
            return chosen.into_iter().fold(0u64, |acc, d| acc * 10 + u64::from(d));
        }
        let remaining = &digits[p + 1..];
        if remaining.len() < 11 - current {
            continue;
        }
        let result = biggest(chosen, remaining, current + 1);
        if result != 0 {
            return result;
        }
    }
    0
}

pub fn puzzle2() {
    let mut result = 0;
    for l in read_lines(3) {
        #[expect(clippy::cast_possible_truncation)]
        let digits: Box<_> = l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
        result += biggest([0; 12], &digits, 0);
    }
    println!("{result}");
}
