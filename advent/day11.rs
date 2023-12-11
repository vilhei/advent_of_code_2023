use std::collections::HashMap;

use crate::utils::{Matrix, Task, TaskError};

pub struct Day11;

#[derive(Debug, PartialEq, Clone, Copy)]
enum SpaceType {
    Galaxy(usize),
    Empty,
}

impl Task for Day11 {
    fn task_part_one(&self, file_content: &str) -> Result<String, TaskError> {
        let m = parse(file_content);

        let empty_rows = empty_rows(&m);
        let (empty_cols, galaxies) = galaxies_and_empty_rows(&m);

        let paths = calc_paths(galaxies, m, empty_rows, empty_cols, 2);
        let res = paths.values().cloned().reduce(|acc, p| acc + p).unwrap();
        Ok(res.to_string())
    }
    fn task_part_two(&self, file_content: &str) -> Result<String, TaskError> {
        let m = parse(file_content);

        let empty_rows = empty_rows(&m);
        let (empty_cols, galaxies) = galaxies_and_empty_rows(&m);

        let paths = calc_paths(galaxies, m, empty_rows, empty_cols, 1_000_000);
        let res = paths.values().cloned().reduce(|acc, p| acc + p).unwrap();
        Ok(res.to_string())
    }

    fn get_day(&self) -> u32 {
        11
    }
}

fn calc_paths(
    galaxies: Vec<(usize, usize)>,
    m: Matrix<SpaceType>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
    expand_factor: usize,
) -> HashMap<(usize, usize), usize> {
    let mut paths = HashMap::new();
    for (gi, gj) in galaxies.iter() {
        for (oi, oj) in galaxies.iter().filter(|c| **c != (*gi, *gj)) {
            if let (SpaceType::Galaxy(gn1), SpaceType::Galaxy(gn2)) = (m[*gi][*gj], m[*oi][*oj]) {
                if !paths.contains_key(&(gn1, gn2)) && !paths.contains_key(&(gn2, gn1)) {
                    let mut empty_compensator = 0;

                    let rs = std::cmp::min(gi, oi);
                    let re = std::cmp::max(gi, oi);

                    empty_rows.iter().for_each(|r| {
                        if (rs..re).contains(&r) {
                            empty_compensator += expand_factor - 1;
                        }
                    });

                    let cs = std::cmp::min(gj, oj);
                    let ce = std::cmp::max(gj, oj);

                    empty_cols.iter().for_each(|c| {
                        if (cs..ce).contains(&c) {
                            empty_compensator += expand_factor - 1;
                        }
                    });

                    let dist = (*oi as isize - *gi as isize).unsigned_abs()
                        + (*oj as isize - *gj as isize).unsigned_abs()
                        + empty_compensator;
                    paths.insert((gn1, gn2), dist);
                }
            } else {
                panic!("Horribly wrong");
            }
        }
    }
    paths
}

fn parse(file_content: &str) -> Matrix<SpaceType> {
    let m = Matrix::from(file_content);
    let mut galaxy_num = 0;
    m.transform_type(|c| {
        if c == '#' {
            let temp = galaxy_num;
            galaxy_num += 1;
            SpaceType::Galaxy(temp)
        } else {
            SpaceType::Empty
        }
    })
}

fn galaxies_and_empty_rows(m: &Matrix<SpaceType>) -> (Vec<usize>, Vec<(usize, usize)>) {
    let mut empty_cols = Vec::new();
    let mut galaxies = Vec::new();
    for j in 0..m.column_len() {
        let mut col_empty = true;
        for i in 0..m.row_len() {
            if m[i][j] != SpaceType::Empty {
                col_empty = false;
                galaxies.push((i, j));
            }
        }
        if col_empty {
            empty_cols.push(j);
        }
    }
    (empty_cols, galaxies)
}

fn empty_rows(m: &Matrix<SpaceType>) -> Vec<usize> {
    let mut empty_rows = Vec::new();
    for i in 0..m.row_len() {
        if m[i].iter().all(|s| *s == SpaceType::Empty) {
            empty_rows.push(i);
        }
    }
    empty_rows
}
