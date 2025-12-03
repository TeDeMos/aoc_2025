use crate::utils::read_string;

fn sum_before(n: u64, power: u64) -> u64 {
    let regular = n * (n + 1) / 2;
    regular * power + regular
}

pub fn puzzle1() {
    let mut result = 0;
    #[expect(clippy::cast_possible_truncation)]
    for s in read_string(2).split(',') {
        let (l, r) = s.split_once('-').unwrap();
        let start =
            if l.len() % 2 == 0 { l.parse().unwrap() } else { 10u64.pow(l.len() as u32) };
        let end = if r.len() % 2 == 0 {
            r.parse().unwrap()
        } else {
            10u64.pow(r.len() as u32 - 1) - 1
        };
        if end < start {
            continue;
        }
        let start_digits = start.ilog10() + 1;
        let end_digits = start.ilog10() + 1;
        assert_eq!(start_digits, end_digits);
        let half_digits = start_digits / 2;
        let power = 10u64.pow(half_digits);
        let (mut start_f, start_b) = (start / power, start % power);
        let (mut end_f, end_b) = (end / power, end % power);
        if start_f < start_b  {
            start_f += 1;
        }
        if end_f > end_b  {
            end_f -= 1;
        }
        if start_f == end_f {
            result += start_f * power + start_f;
        } else if start_f < end_f {
            result += sum_before(end_f, power) - sum_before(start_f - 1, power);
        }
    }
    println!("{result}");
}

pub fn puzzle2() {
    let mut result = 0;
    for s in read_string(2).split(',') {
        let (l, r) = s.split_once('-').unwrap();
        let start: u64 = l.parse().unwrap();
        let end: u64 = r.parse().unwrap();
        for n in start..=end {
            let str = n.to_string();
            let bytes = str.as_bytes();
            for k in 1..=bytes.len() / 2 {
                if bytes.len() % k != 0 {
                    continue;
                }
                let mut chunks = bytes.chunks(k);
                let first = chunks.next().unwrap();
                if chunks.all(|c| c == first) {
                    result += n;
                    break;
                }
            }
            // println!("{str}: {good}");
        }
    }
    print!("{result}");
}