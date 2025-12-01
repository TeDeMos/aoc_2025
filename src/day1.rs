use crate::utils::read_lines;

fn parse_values() -> impl Iterator<Item = i32> {
    read_lines(1)
        .map(|s| {
            let mut chars = s.chars();
            let first = chars.next().unwrap();
            let value: i32 = chars.as_str().parse().unwrap();
            match first {
                'L' => -value,
                'R' => value,
                _ => unreachable!(),
            }
        })
}

pub fn puzzle1() {
    let mut result = 0;
    let mut angle = 50;
    for r in parse_values() {
        angle = (angle + r) % 100;
        if angle == 0 {
            result += 1;
        }
    }
    println!("{result}");
}

pub fn puzzle2() {
    let mut result = 0;
    let mut angle = 50;
    for r in parse_values() {
        let previous = angle;
        result += r.abs() / 100;
        angle = (angle + r).rem_euclid(100);
        if angle == 0 || previous != 0 && (r > 0 && previous > angle || r < 0 && previous < angle) {
            result += 1;
        }
    }
    println!("{result}");
}
