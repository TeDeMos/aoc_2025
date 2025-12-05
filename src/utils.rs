use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::ops::{Index, IndexMut};

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
