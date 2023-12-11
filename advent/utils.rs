use std::{
    fs,
    ops::{Index, IndexMut},
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

pub trait Task {
    #[allow(unused_variables)]
    fn task_part_one(&self, file_content: &str) -> Result<String, TaskError> {
        Err(TaskError::NotImplemented(1))
    }
    #[allow(unused_variables)]
    fn task_part_two(&self, file_content: &str) -> Result<String, TaskError> {
        Err(TaskError::NotImplemented(2))
    }

    fn get_day(&self) -> u32;
}

// pub trait Element:
//     std::ops::Mul + Add<Output = Self> + std::marker::Copy + Default + std::fmt::Display
// {
// }
// impl<T: std::ops::Mul + Add<Output = T> + std::marker::Copy + Default + std::fmt::Display> Element
//     for T
// {
// }

#[derive(Debug, Clone)]
pub struct Matrix<T = char> {
    pub rows: usize,
    pub columns: usize,
    data: Vec<Vec<T>>,
}

impl From<&str> for Matrix<char> {
    fn from(input: &str) -> Self {
        let char_vecs: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();

        Matrix {
            rows: char_vecs.len(),
            columns: char_vecs[0].len(),
            data: char_vecs,
        }
    }
}

// impl<T> Mul for Matrix<T>
// where
//     T: Copy
//         + Mul<Output = T>
//         + std::ops::AddAssign<<T as std::ops::Mul>::Output>
//         + std::default::Default,
//     <T as std::ops::Mul>::Output:
//         std::marker::Copy + std::ops::AddAssign<<T as std::ops::Mul>::Output>,
// {
//     type Output = Matrix<T>;

//     fn mul(self, rhs: Self) -> Self::Output {
//         if self.rows != rhs.columns || self.columns != rhs.rows {
//             panic!(
//                 "Incompatible matrix sizes {}x{}and {}x{}",
//                 self.rows, self.columns, rhs.rows, rhs.columns
//             );
//         }

//         let mut output = Matrix {
//             rows: self.columns,
//             columns: rhs.rows,
//             data: vec![vec![T::default(); rhs.rows]; self.columns],
//         };

//         for (i, row) in self.data.iter().enumerate() {
//             for (j, k) in row.iter().enumerate() {
//                 output[i][j] += *k * rhs[j][i];
//             }
//         }

//         output
//     }
// }

impl<T> Matrix<T> {
    pub fn data(&mut self) -> &mut Vec<Vec<T>> {
        &mut self.data
    }

    pub fn transform_type<U, F>(self, mut f: F) -> Matrix<U>
    where
        F: FnMut(T) -> U,
    {
        Matrix {
            columns: self.columns,
            rows: self.rows,
            data: self
                .data
                .into_iter()
                .map(move |row| {
                    let f1 = &mut f;
                    row.into_iter().map(f1).collect()
                })
                .collect(),
        }
    }

    pub fn parse<F>(input: &str, f: F) -> Self
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

    pub fn column_len(&self) -> usize {
        self.columns
    }
    pub fn row_len(&self) -> usize {
        self.rows
    }
}

impl<T> Index<usize> for Matrix<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

pub fn parse_2d_char_array(input: &str) -> Matrix<char> {
    Matrix::from(input)
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
