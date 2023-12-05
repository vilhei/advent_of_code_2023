use std::{
    fs,
    ops::{Add, Index, IndexMut, Mul},
};

#[derive(Debug)]
pub enum TaskError {
    InvalidFilePath(String),
    NotImplemented(usize),
}

pub fn read_task_input_file(path: &str) -> Result<String, TaskError> {
    let Ok(file_contents) = fs::read_to_string(path) else {
        return Err(TaskError::InvalidFilePath(path.to_string()));
    };
    Ok(file_contents)
}

// pub trait Element:
//     std::ops::Mul + Add<Output = Self> + std::marker::Copy + Default + std::fmt::Display
// {
// }
// impl<T: std::ops::Mul + Add<Output = T> + std::marker::Copy + Default + std::fmt::Display> Element
//     for T
// {
// }

pub struct Matrix<T: Copy = char> {
    rows: usize,
    columns: usize,
    data: Vec<Vec<T>>,
}

impl From<&str> for Matrix<char> {
    fn from(input: &str) -> Self {
        let char_vecs: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();

        Matrix {
            rows: char_vecs[0].len(),
            columns: char_vecs.len(),
            data: char_vecs,
        }
    }
}

impl<T> Mul for Matrix<T>
where
    T: Copy
        + Mul<Output = T>
        + std::ops::AddAssign<<T as std::ops::Mul>::Output>
        + std::default::Default,
    <T as std::ops::Mul>::Output:
        std::marker::Copy + std::ops::AddAssign<<T as std::ops::Mul>::Output>,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.rows != rhs.columns || self.columns != rhs.rows {
            panic!(
                "Incompatible matrix sizes {}x{}and {}x{}",
                self.rows, self.columns, rhs.rows, rhs.columns
            );
        }

        let mut output = Matrix {
            rows: self.columns,
            columns: rhs.rows,
            data: vec![vec![T::default(); rhs.rows]; self.columns],
        };

        for (i, row) in self.data.iter().enumerate() {
            for (j, k) in row.iter().enumerate() {
                output[i][j] += *k * rhs[j][i];
            }
        }

        output
    }
}

impl<T: Copy> Matrix<T> {
    fn data(&self) -> &Vec<Vec<T>> {
        &self.data
    }

    fn transform_type<U, F>(self, f: F) -> Matrix<U>
    where
        U: Copy,
        F: Fn(T) -> U,
    {
        Matrix {
            columns: self.columns,
            rows: self.rows,
            data: self
                .data
                .into_iter()
                .map(|row| row.into_iter().map(&f).collect())
                .collect(),
        }
    }

    fn parse<F>(input: &str, f: F) -> Self
    where
        F: Fn(&str) -> Vec<Vec<T>>,
    {
        let vecs: Vec<Vec<T>> = f(input);

        Matrix {
            rows: vecs[0].len(),
            columns: vecs.len(),
            data: vecs,
        }
    }
}

impl<T: Copy> Index<usize> for Matrix<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T: Copy> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

pub fn parse_2d_char_array(input: &str) -> Matrix<char> {
    Matrix::from(input)
}

pub trait Task {
    fn task_part_one(&self, _file_content: &str) -> Result<String, TaskError> {
        Err(TaskError::NotImplemented(1))
    }
    fn task_part_two(&self, _file_content: &str) -> Result<String, TaskError> {
        Err(TaskError::NotImplemented(2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_2d_char_array() {
        let input = "123\n456\n789";
        let arr = parse_2d_char_array(input);

        let f = |c: char| c.to_digit(10).unwrap();

        let transformed = arr.transform_type(f);

        let res = transformed[0][2];
        assert_eq!(3, res);
    }
}
