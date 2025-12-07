use std::mem;
use crate::utils::read_lines;

fn push_unique(vec: &mut Vec<usize>, item: usize) {
    if vec.last().is_none_or(|&i| i != item) {
        vec.push(item);
    }
}

pub fn puzzle1() {
    let mut lines = read_lines(7).enumerate().filter_map(|(i, l)| (i % 2 == 0).then_some(l));
    let mut beams = vec![lines.next().unwrap().find('S').unwrap()];
    let mut next = Vec::new();
    let mut result = 0;
    for l in lines {
        let bytes= l.as_bytes();
        for b in beams.drain(..) {
            if bytes[b] == b'^' {
                result += 1;
                push_unique(&mut next, b - 1);
                next.push(b + 1);
            } else {
                push_unique(&mut next, b);
            }
        }
        mem::swap(&mut beams, &mut next);
    }
    println!("{result}");
}
pub fn puzzle2() {
    let mut lines = read_lines(7).enumerate().filter_map(|(i, l)| (i % 2 == 0).then_some(l));
    let first = lines.next().unwrap();
    let mut beams = vec![0u64; first.len()].into_boxed_slice();
    beams[first.find('S').unwrap()] = 1;
    let mut next = vec![0; first.len()].into_boxed_slice();
    for l in lines {
        let bytes= l.as_bytes();
        for (n, &b) in beams.iter().enumerate() {
            if b == 0 {
                continue;
            }
            if bytes[n] == b'^' {
                next[n - 1] += b;
                next[n + 1] += b;
            } else {
                next[n] += b;
            }
        }
        mem::swap(&mut beams, &mut next);
        next.fill(0);
    }
    let result: u64 = beams.into_iter().sum();
    println!("{result}");
}
