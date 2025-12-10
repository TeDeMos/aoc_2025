use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::ops::{Add, Index, IndexMut, Mul, Sub};

fn path(day: usize) -> String {
    format!("/home/tedem/dev/RustroverProjects/aoc_2025/input/{day}.txt")
}

pub fn read_string(day: usize) -> String { fs::read_to_string(path(day)).unwrap() }

pub fn read_lines(day: usize) -> impl Iterator<Item = String> {
    File::open_buffered(path(day)).unwrap().lines().map_while(Result::ok)
}

pub struct Matrix<T> {
    columns: usize,
    rows: usize,
    data: Box<[T]>,
}

#[derive(Debug)]
pub struct InconsistentRows;

impl<T> Matrix<T> {
    const OFFSETS: [(isize, isize); 8] =
        [(-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0)];

    pub fn columns(&self) -> usize { self.columns }

    pub fn rows(&self) -> usize { self.rows }

    pub fn new_default(columns: usize, rows: usize) -> Self
    where T: Default + Clone {
        Self { columns, rows, data: vec![T::default(); columns * rows].into_boxed_slice() }
    }

    pub fn from_row_iter<I>(rows: I) -> Result<Self, InconsistentRows>
    where
        I: IntoIterator,
        I::Item: IntoIterator<Item = T>,
    {
        let mut row_iter = rows.into_iter();
        let mut result: Vec<_> = match row_iter.next() {
            Some(row) => row.into_iter().collect(),
            None => return Ok(Self { columns: 0, rows: 0, data: Box::new([]) }),
        };
        let columns = result.len();
        (columns > 0).ok_or(InconsistentRows)?;
        let mut rows = 1;
        for row in row_iter {
            let previous_len = result.len();
            result.extend(row);
            (result.len() - previous_len == columns).ok_or(InconsistentRows)?;
            rows += 1;
        }
        Ok(Self { columns, rows, data: result.into_boxed_slice() })
    }

    pub fn get(&self, column: usize, row: usize) -> Option<&T> {
        (column < self.columns && row < self.rows).then(|| &self.data[row * self.columns + column])
    }

    pub fn get_mut(&mut self, column: usize, row: usize) -> Option<&mut T> {
        (column < self.columns && row < self.rows)
            .then(|| &mut self.data[row * self.columns + column])
    }

    pub fn neighbours(&self, column: usize, row: usize) -> impl Iterator<Item = &T> {
        Self::OFFSETS.into_iter().filter_map(move |(xd, yd)| {
            self.get(column.checked_add_signed(xd)?, row.checked_add_signed(yd)?)
        })
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, (column, row): (usize, usize)) -> &Self::Output {
        assert!(column < self.columns && row < self.rows);
        &self.data[row * self.columns + column]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (column, row): (usize, usize)) -> &mut Self::Output {
        assert!(column < self.columns && row < self.rows);
        &mut self.data[row * self.columns + column]
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn map2(self, other: Self, mut f: impl FnMut(T, T) -> T) -> Self {
        Self { x: f(self.x, other.x), y: f(self.y, other.y) }
    }

    pub fn product(self) -> T::Output
    where T: Mul {
        self.x * self.y
    }
}

impl<T: Add<Output = T> + Copy> Add<T> for Vec2<T> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output { Self { x: self.x + rhs, y: self.y + rhs } }
}

impl<T> From<[T; 2]> for Vec2<T> {
    fn from([x, y]: [T; 2]) -> Self { Self { x, y } }
}

pub fn sort2_by_key<T, K: Ord>(a: T, b: T, mut k: impl FnMut(&T) -> K) -> [T; 2] {
    if k(&a) <= k(&b) { [a, b] } else { [b, a] }
}