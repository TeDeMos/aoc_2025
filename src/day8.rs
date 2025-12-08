use std::iter;
use crate::utils::read_lines;

struct JBox {
    x: u64,
    y: u64,
    z: u64,
}

impl From<[u64; 3]> for JBox {
    fn from([x, y, z]: [u64; 3]) -> Self { Self { x, y, z } }
}

impl JBox {
    #[expect(clippy::cast_precision_loss)]
    fn distance(&self, other: &Self) -> f64 {
        (self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)) as _
    }
}

fn replace(slice: &mut [usize], orig: usize, target: usize) {
    for x in slice {
        if *x == orig {
            *x = target;
        }
    }
}

fn get_data() -> (Vec<JBox>, Vec<(f64, usize, usize)>) {
    let boxes: Vec<JBox> = read_lines(8)
        .map(|l| l.split(',').map(|s| s.parse().unwrap()).collect_array().unwrap().into())
        .collect();
    let boxes_ref = &boxes;
    let mut distances: Vec<_> = (0..boxes.len() - 1)
        .flat_map(|l| {
            (l + 1..boxes.len()).map(move |r| (boxes_ref[l].distance(&boxes_ref[r]), l, r))
        })
        .collect();
    distances.sort_unstable_by(|&(l, ..), (r, ..)| l.total_cmp(r));
    (boxes, distances)
}

pub fn puzzle1() {
    let (boxes, mut distances) = get_data();
    distances.truncate(1000);
    let mut circuit_idx: Box<_> = (0..boxes.len()).collect();
    for (_, l, r) in distances {
        if circuit_idx[l] == circuit_idx[r] {
            continue;
        }
        let orig = circuit_idx[l];
        let target = circuit_idx[r];
        replace(&mut circuit_idx, orig, target);
    }
    circuit_idx.sort_unstable();
    let mut max = [(usize::MAX, 0); 3];
    let mut current = circuit_idx[0];
    let mut current_count = 1;
    for c in circuit_idx.into_iter().skip(1).chain(iter::once(0)) {
        if c == current {
            current_count += 1;
        } else {
            if let Some(i) = max.iter().position(|&(_, count)| count < current_count) {
                max[i..].rotate_right(1);
                max[i] = (current, current_count);
            }
            current_count = 1;
            current = c;
        }
    }
    let result: i32 = max.into_iter().map(|(_, count)| count).product();
    println!("{result}");
}

fn replace_and_check(slice: &mut [usize], orig: usize, target: usize) -> bool{
    let mut unique = true;
    let mut first = None;
    for x in slice {
        if *x == orig {
            *x = target;
        }
        match first {
            Some(f) => unique = unique && f == *x,
            None => first = Some(*x),
        }
    }
    unique
}

pub fn puzzle2() {
    let (boxes, distances) = get_data();
    let mut circuit_idx: Box<_> = (0..boxes.len()).collect();
    let mut result = 0;
    for (_, l, r) in distances {
        if circuit_idx[l] == circuit_idx[r] {
            continue;
        }
        let orig = circuit_idx[l];
        let target = circuit_idx[r];
        if replace_and_check(&mut circuit_idx, orig, target) {
            result = boxes[l].x * boxes[r].x;
            break;
        }
    }
    println!("{result}");
}