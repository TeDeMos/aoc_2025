use crate::utils::{TupleExt, read_lines};

pub fn puzzle1() {
    let mut iter = read_lines(12).peekable();
    let mut areas: Vec<usize> = Vec::new();
    while !iter.peek().unwrap().contains('x') {
        areas.push(
            iter.by_ref()
                .skip(1)
                .take_while(|s| !s.is_empty())
                .map(|s| s.bytes().filter(|&b| b == b'#').count())
                .sum(),
        );
    }
    let result: u32 = iter
        .map(|s| {
            let (l, r) = s.split_once(": ").unwrap();
            let available: usize = l
                .split_once('x')
                .unwrap()
                .into_iter()
                .map(|s| s.parse::<usize>().unwrap())
                .product();
            let required: usize = r
                .split(' ')
                .enumerate()
                .map(|(i, n)| {
                    let n: usize = n.parse().unwrap();
                    areas[i] * n
                })
                .sum();
            u32::from(available >= required)
        })
        .sum();
    println!("{result}");
}
