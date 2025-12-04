use crate::utils::{Matrix, read_lines};

fn parse_grid() -> Matrix<bool> {
    let iter = read_lines(4).map(|l| {
        l.into_chars().map(|c| match c {
            '@' => true,
            '.' => false,
            _ => unreachable!(),
        })
    });
    Matrix::from_row_iter(iter).unwrap()
}

pub fn puzzle1() {
    let grid = parse_grid();
    let mut result = 0;
    for x in 0..grid.columns() {
        for y in 0..grid.rows() {
            if grid[(x, y)] && grid.neighbours(x, y).filter(|&&b| b).count() < 4 {
                result += 1;
            }
        }
    }
    println!("{result}");
}

pub fn puzzle2() {
    let mut grid = parse_grid();
    let mut result = 0;
    loop {
        let mut changed = false;
        for x in 0..grid.columns() {
            for y in 0..grid.rows() {
                if grid[(x, y)] && grid.neighbours(x, y).filter(|&&b| b).count() < 4 {
                    result += 1;
                    grid[(x, y)] = false;
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }
    println!("{result}");
}