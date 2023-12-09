use crate::utils::{Task, TaskError};

pub struct Day9;

impl Task for Day9 {
    fn task_part_one(&self, file_content: &str) -> Result<String, TaskError> {
        let sequences = parse(file_content);

        let mut total = 0;
        for sequence in sequences {
            let mut diffs = calc_diffs_until_zeros(sequence);
            total += solve_next(&mut diffs)
        }
        Ok(total.to_string())
    }
    fn task_part_two(&self, file_content: &str) -> Result<String, TaskError> {
        let sequences = parse_rev(file_content);

        let mut total = 0;
        for sequence in sequences {
            let mut diffs = calc_diffs_until_zeros(sequence);
            total += solve_next(&mut diffs);
        }
        Ok(total.to_string())
    }

    fn get_day(&self) -> u32 {
        9
    }
}

fn solve_next(diffs: &mut Vec<Vec<i64>>) -> i64 {
    for i in (0..diffs.len() - 1).rev() {
        let new = diffs[i + 1].last().unwrap() + diffs[i].last().unwrap();
        diffs[i].push(new);
    }
    *diffs.first().unwrap().last().unwrap()
}

fn calc_diffs_until_zeros(mut curr: Vec<i64>) -> Vec<Vec<i64>> {
    let mut diffs = Vec::new();
    diffs.push(curr.clone());
    loop {
        let diff = curr.windows(2).map(|a| a[1] - a[0]).collect::<Vec<i64>>();
        diffs.push(diff.clone());
        if diff.iter().all(|e| *e == 0) {
            break;
        }
        curr = diff;
    }
    diffs
}

fn parse(file_content: &str) -> Vec<Vec<i64>> {
    file_content
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>()
}
fn parse_rev(file_content: &str) -> Vec<Vec<i64>> {
    file_content
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .rev()
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>()
}
