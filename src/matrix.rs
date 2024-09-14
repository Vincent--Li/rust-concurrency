use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Mul},
};

use anyhow::{anyhow, Result};

pub struct Matrix<T> {
    pub row: usize,
    pub col: usize,
    pub data: Vec<T>,
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T>,
{
    if a.col != b.row {
        return Err(anyhow!("Invalid matrix size"));
    }

    let mut data = vec![T::default(); a.row * b.col];
    for i in 0..a.row {
        for j in 0..b.col {
            for k in 0..a.col {
                data[i * b.col + j] += a.data[i * a.col + k] * b.data[k * b.col + j];
            }
        }
    }

    let result = Matrix {
        row: a.row,
        col: b.col,
        data,
    };
    Ok(result)
}

impl<T: Debug> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
        Self {
            row,
            col,
            data: data.into(),
        }
    }
}

impl<T> Display for Matrix<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.row {
            for j in 0..self.col {
                write!(f, "{} ", self.data[i * self.col + j])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> Debug for Matrix<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_multiply() -> Result<()> {
        // 3 * 2 matrix multiply 2 * 3 matrix
        let a = Matrix::new(vec![1, 2, 3, 4, 5, 6], 3, 2);
        let b = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let c = multiply(&a, &b)?;
        assert_eq!(c.row, 3);
        assert_eq!(c.col, 3);
        assert_eq!(c.data, vec![9, 12, 15, 19, 26, 33, 29, 40, 51]);
        println!("{}", c);
        Ok(())
    }
}
