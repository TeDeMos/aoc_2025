use std::ops::RangeInclusive;

use crate::utils::read_lines;

fn parse_ranges(lines: impl Iterator<Item = String>) -> impl Iterator<Item = RangeInclusive<u64>> {
    lines.map_while(|l| {
        if l.is_empty() {
            return None;
        }
        let (l, r) = l.split_once('-').unwrap();
        let start: u64 = l.parse().unwrap();
        let end: u64 = r.parse().unwrap();
        Some(start..=end)
    })
}

pub fn puzzle1() {
    let mut lines = read_lines(5);
    let ranges: Vec<_> = parse_ranges(lines.by_ref()).collect();
    let result = lines.filter(|l| {
        let n = l.parse().unwrap();
        ranges.iter().any(|r| r.contains(&n))
    }).count();
    println!("{result}");
}

pub fn puzzle2() {
    let mut ranges: Vec<RangeInclusive<u64>> = Vec::new();
    'outer: for mut a in parse_ranges(read_lines(5)) {
        let mut i = 0;
        while i < ranges.len() {
            let b = &ranges[i];
            if a.end() < b.start() || b.end() < a.start() {
                // Disjoint
                i += 1;
            } else if a.contains(b.start()) && a.end() < b.end() {
                // Partial overlap 1
                a = *a.start()..=*b.end();
                ranges.swap_remove(i);
            } else if b.contains(a.start()) && b.end() < a.end() {
                // Partial overlap 2
                a = *b.start()..=*a.end();
                ranges.swap_remove(i);
            } else if b.start() <= a.start() && a.end() <= b.end() {
                // A fully covered by B
                continue 'outer;
            } else if a.start() <= b.start() && b.end() <= a.end() {
                // B fully covered by A
                ranges.swap_remove(i);
            }
        }
        ranges.push(a);
    }
    let result: u64 = ranges.into_iter().map(|r| *r.end() - r.start() + 1).sum();
    println!("{result}");
}
