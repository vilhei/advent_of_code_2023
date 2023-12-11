use crate::utils::{Task, TaskError};

pub struct Day9;

impl Task for Day9 {
    fn task_part_one(&self, file_content: &str) -> Result<String, TaskError> {
        let sequences = parse(file_content);

        let mut total = 0;
        for sequence in sequences {
            let diffs = calc_diffs_until_zeros(sequence);
            total += diffs.iter().sum::<i64>();
        }
        Ok(total.to_string())
    }
    fn task_part_two(&self, file_content: &str) -> Result<String, TaskError> {
        let sequences = parse_rev(file_content);

        let mut total = 0;
        for sequence in sequences {
            let diffs = calc_diffs_until_zeros(sequence);
            total += diffs.iter().sum::<i64>();
        }
        Ok(total.to_string())
    }

    fn get_day(&self) -> u32 {
        9
    }
}

fn calc_diffs_until_zeros(mut curr: Vec<i64>) -> Vec<i64> {
    let mut diffs: Vec<i64> = Vec::new();
    diffs.push(*curr.last().unwrap());
    loop {
        let diff = curr.windows(2).map(|a| a[1] - a[0]).collect::<Vec<i64>>();
        diffs.push(*diff.last().unwrap());
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
