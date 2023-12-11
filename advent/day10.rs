use itertools::Itertools;

use crate::utils::{Matrix, Task, TaskError};

pub struct Day10;

enum Dir {
    Left,
    Up,
    Right,
    Down,
}

impl Task for Day10 {
    #[allow(unused_variables)]
    fn task_part_one(&self, file_content: &str) -> Result<String, TaskError> {
        let matrix = Matrix::from(file_content);

        // Find start
        let mut si = 0;
        let mut sj = 0;

        for i in 0..matrix.row_len() {
            for j in 0..matrix.column_len() {
                if matrix[i][j] == 'S' {
                    si = i;
                    sj = j;
                    break;
                }
            }
        }
        let mut ci = si;
        let mut cj = sj;
        let mut loop_path = Vec::new();
        loop_path.push((ci, cj));
        // let left = '0';
        // let up = '0';
        // let right = '0';
        // let down = '0';

        let mut count = 0;
        let mut path = '0';

        let mut prev = Dir::Down;

        determine_start_dir(&mut cj, &matrix, &mut ci, &mut path, &mut prev);

        count += 1;

        loop {
            // Choose one path and
            match path {
                '|' => {
                    count += 1;
                    match prev {
                        Dir::Up => {
                            ci += 1;
                            path = matrix[ci][cj]
                        }
                        Dir::Down => {
                            ci -= 1;
                            path = matrix[ci][cj]
                        }
                        _ => panic!("Not possible"),
                    }
                }
                '-' => {
                    count += 1;
                    match prev {
                        Dir::Left => {
                            cj += 1;
                            path = matrix[ci][cj]
                        }
                        Dir::Right => {
                            cj -= 1;
                            path = matrix[ci][cj]
                        }
                        _ => panic!("Not possible"),
                    }
                }
                'L' => {
                    count += 1;
                    match prev {
                        Dir::Up => {
                            prev = Dir::Left;
                            cj += 1;
                            path = matrix[ci][cj]
                        }
                        Dir::Right => {
                            prev = Dir::Down;
                            ci -= 1;
                            path = matrix[ci][cj]
                        }
                        _ => panic!("Not possible"),
                    }
                }
                'J' => {
                    count += 1;
                    match prev {
                        Dir::Up => {
                            prev = Dir::Right;
                            cj -= 1;
                            path = matrix[ci][cj]
                        }
                        Dir::Left => {
                            prev = Dir::Down;
                            ci -= 1;
                            path = matrix[ci][cj]
                        }
                        _ => panic!("Not possible"),
                    }
                }
                '7' => {
                    count += 1;
                    match prev {
                        Dir::Down => {
                            prev = Dir::Right;
                            cj -= 1;
                            path = matrix[ci][cj]
                        }
                        Dir::Left => {
                            prev = Dir::Up;
                            ci += 1;
                            path = matrix[ci][cj]
                        }
                        _ => panic!("Not possible"),
                    }
                }
                'F' => {
                    count += 1;
                    match prev {
                        Dir::Down => {
                            prev = Dir::Left;
                            cj += 1;
                            path = matrix[ci][cj]
                        }
                        Dir::Right => {
                            prev = Dir::Up;
                            ci += 1;
                            path = matrix[ci][cj]
                        }
                        _ => panic!("Not possible"),
                    }
                }
                'S' => break,
                _ => panic!("Not possible"),
            }
            loop_path.push((ci, cj));
            // println!("{left},{up},{right}, {down}");
        }
        println!("{}", count / 2);
        Ok((count / 2).to_string())
    }
    #[allow(unused_variables)]
    fn task_part_two(&self, file_content: &str) -> Result<String, TaskError> {
        let matrix = Matrix::from(file_content);

        // Find start
        let mut si = 0;
        let mut sj = 0;

        for i in 0..matrix.row_len() {
            for j in 0..matrix.column_len() {
                if matrix[i][j] == 'S' {
                    si = i;
                    sj = j;
                    break;
                }
            }
        }
        let mut ci = si;
        let mut cj = sj;
        let mut loop_path = Vec::new();
        loop_path.push((ci, cj));

        // let mut count = 0;
        let mut path = '0';

        let mut prev = Dir::Down;

        determine_start_dir(&mut cj, &matrix, &mut ci, &mut path, &mut prev);

        // count += 1;

        loop {
            // Choose one path and
            loop_path.push((ci, cj));
            // let (prev_i, prev_j) = (ci, cj);
            match path {
                '|' => match prev {
                    Dir::Up => {
                        ci += 1;
                        path = matrix[ci][cj]
                    }
                    Dir::Down => {
                        ci -= 1;
                        path = matrix[ci][cj]
                    }
                    _ => panic!("Not possible"),
                },
                '-' => match prev {
                    Dir::Left => {
                        cj += 1;
                        path = matrix[ci][cj]
                    }
                    Dir::Right => {
                        cj -= 1;
                        path = matrix[ci][cj]
                    }
                    _ => panic!("Not possible"),
                },
                'L' => match prev {
                    Dir::Up => {
                        prev = Dir::Left;
                        cj += 1;
                        path = matrix[ci][cj]
                    }
                    Dir::Right => {
                        prev = Dir::Down;
                        ci -= 1;
                        path = matrix[ci][cj]
                    }
                    _ => panic!("Not possible"),
                },
                'J' => match prev {
                    Dir::Up => {
                        prev = Dir::Right;
                        cj -= 1;
                        path = matrix[ci][cj]
                    }
                    Dir::Left => {
                        prev = Dir::Down;
                        ci -= 1;
                        path = matrix[ci][cj]
                    }
                    _ => panic!("Not possible"),
                },
                '7' => match prev {
                    Dir::Down => {
                        prev = Dir::Right;
                        cj -= 1;
                        path = matrix[ci][cj]
                    }
                    Dir::Left => {
                        prev = Dir::Up;
                        ci += 1;
                        path = matrix[ci][cj]
                    }
                    _ => panic!("Not possible"),
                },
                'F' => match prev {
                    Dir::Down => {
                        prev = Dir::Left;
                        cj += 1;
                        path = matrix[ci][cj]
                    }
                    Dir::Right => {
                        prev = Dir::Up;
                        ci += 1;
                        path = matrix[ci][cj]
                    }
                    _ => panic!("Not possible"),
                },
                'S' => break,
                _ => panic!("Not possible"),
            }
            // loop_path.push((prev_i, prev_j));
            // println!("{left},{up},{right}, {down}");
        }

        // println!(
        //     "{:?}",
        //     loop_path.iter().map(|a| a.1).collect::<Vec<usize>>()
        // );
        let area = shoe_lace(&loop_path);

        Ok(area.to_string())
    }

    fn get_day(&self) -> u32 {
        10
    }
}

fn shoe_lace(loop_path: &[(usize, usize)]) -> usize {
    let mut area = 0;
    // let loop_path = [(1, 6), (3, 1), (7, 2), (4, 4), (8, 5), (1, 6)];

    for ((x1, y1), (x2, y2)) in loop_path
        .iter()
        // .chain(iter::once(loop_path.first().unwrap()))
        .tuple_windows()
    {
        let det = *x1 as isize * *y2 as isize - *x2 as isize * *y1 as isize;
        if det.abs() > 1 {
            area += det;
        }
    }

    let area = area.unsigned_abs() / 2;
    let area = loop_path.len() - 1 + area / 2 - 1;
    area
}

fn determine_start_dir(
    cj: &mut usize,
    matrix: &Matrix,
    ci: &mut usize,
    path: &mut char,
    prev: &mut Dir,
) {
    if *cj > 0 && !(".|J7".contains(matrix[*ci][*cj - 1])) {
        // Left
        *path = matrix[*ci][*cj - 1];
        *prev = Dir::Right;
        *cj -= 1;
        return;
    }
    if *cj + 1 < matrix.column_len() && !(".|LF".contains(matrix[*ci][*cj + 1])) {
        // Right
        *path = matrix[*ci][*cj + 1];
        *prev = Dir::Left;
        *cj += 1;
        return;
    }
    if *ci > 0 && !(".-LJ".contains(matrix[*ci - 1][*cj])) {
        // Top
        *path = matrix[*ci - 1][*cj];
        *ci -= 1;
        *prev = Dir::Down;
        return;
    }
    if *ci + 1 < matrix.row_len() && !(".-7F".contains(matrix[*ci + 1][*cj])) {
        // Down
        *path = matrix[*ci + 1][*cj];
        *ci += 1;
        *prev = Dir::Up;
    }
}
