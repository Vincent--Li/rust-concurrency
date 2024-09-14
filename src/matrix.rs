use crate::{dot_product, Vector};
use anyhow::{anyhow, Result};
use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Mul},
    sync::mpsc,
    thread,
};

const NUN_THREADS: usize = 16;

pub struct Matrix<T> {
    pub row: usize,
    pub col: usize,
    pub data: Vec<T>,
}

pub struct MsgInput<T> {
    idx: usize,
    row: Vector<T>,
    col: Vector<T>,
}

pub struct MsgOutput<T> {
    idx: usize,
    value: T,
}

pub struct Msg<T> {
    input: MsgInput<T>,
    sender: oneshot::Sender<MsgOutput<T>>,
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T> + Send + 'static,
{
    if a.col != b.row {
        return Err(anyhow!("Invalid matrix size"));
    }

    // map/reduce: map phase
    let senders = (0..NUN_THREADS)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in rx {
                    let value = dot_product(msg.input.row, msg.input.col)?;
                    if let Err(e) = msg.sender.send(MsgOutput::new(msg.input.idx, value)) {
                        eprintln!("Failed to send message: {:?}", e);
                    };
                }
                Ok::<_, anyhow::Error>(())
            });
            tx
        })
        .collect::<Vec<_>>();

    let matrix_len = a.row * b.col;
    let mut data = vec![T::default(); matrix_len];
    let mut receivers = Vec::with_capacity(matrix_len);
    for i in 0..a.row {
        for j in 0..b.col {
            let row = Vector::new(&a.data[i * a.col..(i + 1) * a.col]);
            let col_data = b.data[j..]
                .iter()
                .step_by(b.col)
                .copied()
                .collect::<Vec<_>>();
            let col = Vector::new(col_data);
            let idx = i * b.col + j;
            let input = MsgInput::new(idx, row, col);
            let (tx, rx) = oneshot::channel();
            let msg = Msg::new(input, tx);
            if let Err(e) = senders[idx % NUN_THREADS].send(msg) {
                eprintln!("Failed to send message: {:?}", e);
            };
            receivers.push(rx);
        }
    }

    // mat/reduce: reduce phase
    for rx in receivers {
        let output = rx.recv()?;
        data[output.idx] = output.value;
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

impl<T> Mul for Matrix<T>
where
    T: Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T> + Send + 'static,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        multiply(&self, &rhs).expect("Failed to multiply matrices")
    }
}

impl<T> MsgInput<T> {
    fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> Self {
        Self { idx, row, col }
    }
}

impl<T> MsgOutput<T> {
    fn new(idx: usize, value: T) -> Self {
        Self { idx, value }
    }
}

impl<T> Msg<T> {
    fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Self { input, sender }
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
        let c = a * b;
        assert_eq!(c.row, 3);
        assert_eq!(c.col, 3);
        assert_eq!(c.data, vec![9, 12, 15, 19, 26, 33, 29, 40, 51]);
        println!("{}", c);
        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_matrix_multiply_with_empty_matrix() {
        let a = Matrix::new(vec![1, 2, 3, 4, 5, 6], 3, 2);
        let b = Matrix::new(vec![], 0, 0);
        let _ = a * b;
    }
}
