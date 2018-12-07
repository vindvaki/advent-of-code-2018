use std::collections::HashMap;

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

pub struct Mat<T: Clone> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T: Clone> Mat<T> {
    pub fn new(rows: usize, cols: usize, zero: T) -> Mat<T> {
        Mat {
            rows: rows,
            cols: cols,
            data: core::iter::repeat(zero).take(rows * cols).collect(),
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
}