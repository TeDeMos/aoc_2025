use std::cmp::Ordering;
use std::fmt::{Debug, Formatter, Write};
use std::fs::File;
use std::io::BufRead;
use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, MulAssign, Neg, Sub};
use std::{fmt, fs, iter};

fn path(day: usize) -> String {
    format!("/home/tedem/dev/RustroverProjects/aoc_2025/input/{day}.txt")
}

pub fn read_string(day: usize) -> String { fs::read_to_string(path(day)).unwrap() }

pub fn read_lines(day: usize) -> impl Iterator<Item = String> {
    File::open_buffered(path(day)).unwrap().lines().map_while(Result::ok)
}

#[derive(Clone)]
pub struct Matrix<T> {
    columns: usize,
    rows: usize,
    data: Box<[T]>,
}

impl<T: Debug> Debug for Matrix<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (i, r) in self.iter_rows().enumerate() {
            f.write_char(if i == 0 { '[' } else { ' ' })?;
            for i in r {
                write!(f, " {i:>3?}")?;
            }
            f.write_char(if i == self.rows - 1 { ']' } else { '\n' })?;
        }
        Ok(())
    }
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

    pub fn get_row_mut(&mut self, row: usize) -> Option<&mut [T]> {
        (row < self.rows).then(|| &mut self.data[row * self.columns..(row + 1) * self.columns])
    }

    pub fn iter_rows(&self) -> impl Iterator<Item = &[T]> { self.data.chunks(self.columns) }

    pub fn swap_rows(&mut self, a: usize, b: usize) {
        assert!(a < self.rows && b < self.rows);
        if a == b {
            return;
        }
        let [first_idx, second_idx] = sort2(a * self.columns, b * self.columns);
        let (first, second) = self.data.split_at_mut(second_idx);
        first[first_idx..first_idx + self.columns].swap_with_slice(&mut second[..self.columns]);
    }

    pub fn scale_row<U: Copy>(&mut self, row: usize, scale: U)
    where T: MulAssign<U> {
        self.get_row_mut(row).unwrap().iter_mut().for_each(|v| *v *= scale);
    }

    pub fn add_rows<U: Copy>(&mut self, target: usize, source: usize, scale: U)
    where
        T: Mul<U> + Copy,
        T: AddAssign<T::Output>,
    {
        for i in 0..self.columns {
            let value = self[(i, source)] * scale;
            self[(i, target)] += value;
        }
    }

    pub fn map<U>(self, f: impl FnMut(T) -> U) -> Matrix<U> {
        let data = self.data.into_iter().map(f).collect();
        Matrix { columns: self.columns, rows: self.rows, data }
    }

    pub fn with_new_default_row(&self) -> Self
    where T: Default + Clone {
        let mut new_data = Vec::with_capacity(self.data.len() + self.columns);
        new_data.extend_from_slice(&self.data);
        new_data.extend(iter::repeat_with(T::default).take(self.columns));
        Self { columns: self.columns, rows: self.rows + 1, data: new_data.into_boxed_slice() }
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

pub fn sort2<T: Ord>(a: T, b: T) -> [T; 2] { if a <= b { [a, b] } else { [b, a] } }

pub fn sort2_by_key<T, K: Ord>(a: T, b: T, mut k: impl FnMut(&T) -> K) -> [T; 2] {
    if k(&a) <= k(&b) { [a, b] } else { [b, a] }
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rational {
    numerator: i64,
    denominator: i64,
}

impl Rational {
    pub const ONE: Self = Self { numerator: 1, denominator: 1 };
    pub const ZERO: Self = Self { numerator: 0, denominator: 1 };

    fn reduce(&mut self) {
        let g = gcd(self.numerator, self.denominator);
        self.numerator /= g;
        self.denominator /= g;
        if self.denominator < 0 {
            self.numerator *= -1;
            self.denominator *= -1;
        }
    }

    pub fn is_integer(self) -> bool { self.denominator == 1 }

    pub fn ceil(self) -> i64 {
        if self.is_integer() {
            self.numerator
        } else if self.numerator > 0 {
            self.numerator / self.denominator + 1
        } else {
            self.numerator / self.denominator
        } 
    }
}

impl Debug for Rational {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

impl Default for Rational {
    fn default() -> Self { Self::ZERO }
}

impl From<i64> for Rational {
    fn from(value: i64) -> Self { Self { numerator: value, denominator: 1 } }
}

impl AddAssign for Rational {
    fn add_assign(&mut self, rhs: Self) {
        self.numerator = self.numerator * rhs.denominator + self.denominator * rhs.numerator;
        self.denominator *= rhs.denominator;
        self.reduce();
    }
}

impl Mul for Rational {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Self {
            numerator: self.numerator * rhs.numerator,
            denominator: self.denominator * rhs.denominator,
        };
        result.reduce();
        result
    }
}

impl MulAssign for Rational {
    fn mul_assign(&mut self, rhs: Self) {
        self.numerator *= rhs.numerator;
        self.denominator *= rhs.denominator;
        self.reduce();
    }
}

impl Div for Rational {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let mut result = Self {
            numerator: self.numerator * rhs.denominator,
            denominator: self.denominator * rhs.numerator,
        };
        result.reduce();
        result
    }
}

impl Neg for Rational {
    type Output = Self;

    fn neg(self) -> Self::Output { Self { numerator: -self.numerator, ..self } }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Ord for Rational {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.numerator * other.denominator).cmp(&(self.denominator * other.numerator))
    }
}
