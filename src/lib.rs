#[macro_use]
extern crate error_chain;

use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};

pub mod elfcode;

mod errors {
    error_chain! {
        errors {
            ParseValueError(s: String) {
                description("unable to parse value"),
                display("unable to parse value: '{}'", s)
            }

            ParseMatrixDimensionsError(dim: String, expected: usize, actual: usize) {
                description("invalid matrix dimensions"),
                display("invalid number of {}: Expected {} but got {}", dim, expected, actual)
            }
        }
    }
}

use errors::*;

pub fn count_by_value<'a, T: 'a, I>(data: I) -> HashMap<T, u32>
where
    I: Iterator<Item = T>,
    T: Eq + std::hash::Hash,
{
    let mut result = HashMap::new();
    for value in data {
        let count: u32 = *result.get(&value).unwrap_or(&0);
        result.insert(value, count + 1);
    }
    return result;
}

#[derive(Debug, Clone)]
pub struct Mat<T: Clone> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T: Clone + Hash> Hash for Mat<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.rows.hash(state);
        self.cols.hash(state);
        self.data.hash(state);
    }
}

impl<T: Clone + PartialEq> PartialEq for Mat<T> {
    fn eq(&self, other: &Self) -> bool {
        self.rows == other.rows && self.cols == other.cols && self.data == other.data
    }
}
impl<T: Clone + Eq + PartialEq> Eq for Mat<T> {}

impl<T: Clone> Mat<T> {
    pub fn new(rows: usize, cols: usize, zero: T) -> Mat<T> {
        Mat {
            rows: rows,
            cols: cols,
            data: vec![zero; rows * cols],
        }
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        debug_assert!(row < self.rows && col < self.cols);
        &self.data[row + col * self.rows]
    }

    pub fn set(&mut self, row: usize, col: usize, val: T) {
        debug_assert!(row < self.rows && col < self.cols);
        self.data[row + col * self.rows] = val;
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T> {
        self.data.iter()
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }
}

impl<T: Clone + fmt::Display> fmt::Display for Mat<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                write!(f, "{}", self.get(row, col))?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl<T: Clone + fmt::Debug + Default + std::str::FromStr> std::str::FromStr for Mat<T> {
    type Err = errors::Error;

    fn from_str(data: &str) -> Result<Self> {
        let rows = data.lines().count();
        let cols = data.lines().next().unwrap_or("").len();
        let mut mat = Mat::new(rows, cols, T::default());
        let mut last_row = 0;
        for (row, line) in data.lines().enumerate() {
            if row >= rows {
                bail!(ErrorKind::ParseMatrixDimensionsError(
                    "rows".to_owned(),
                    row,
                    rows
                ));
            }
            let mut last_col = 0;
            for (col, c) in line.chars().enumerate() {
                let s = &c.to_string();
                let v: T = s
                    .parse()
                    .map_err(|_| ErrorKind::ParseValueError(s.to_owned()))?;
                if col >= cols {
                    bail!(ErrorKind::ParseMatrixDimensionsError(
                        "columns".to_owned(),
                        col,
                        cols
                    ));
                }
                mat.set(row, col, v);
                last_col = col;
            }
            if last_col + 1 != cols {
                bail!(ErrorKind::ParseMatrixDimensionsError(
                    "columns".to_owned(),
                    last_col,
                    cols
                ));
            }
            last_row = row;
        }
        if last_row + 1 != rows {
            bail!(ErrorKind::ParseMatrixDimensionsError(
                "rows".to_owned(),
                last_row,
                rows
            ));
        }
        Ok(mat)
    }
}
