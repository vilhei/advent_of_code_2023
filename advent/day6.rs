use crate::utils::{Task, TaskError};

pub struct Day6;

impl Task for Day6 {
    fn task_part_one(&self, file_content: &str) -> Result<String, TaskError> {
        let mut lines = file_content.lines();
        let times = lines
            .next()
            .unwrap()
            .strip_prefix("Time:")
            .unwrap()
            .split_ascii_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let records = lines
            .next()
            .unwrap()
            .strip_prefix("Distance:")
            .unwrap()
            .split_ascii_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let mut total = 1;
        for (time, record) in times.into_iter().zip(records) {
            total *= calc_win_scenario_count(time, record);
        }
        Ok(total.to_string())
    }
    fn task_part_two(&self, file_content: &str) -> Result<String, TaskError> {
        let mut lines = file_content.lines();
        let time: usize = lines
            .next()
            .unwrap()
            .strip_prefix("Time:")
            .unwrap()
            .split_ascii_whitespace()
            .fold("".to_string(), |acc, s| acc + s)
            .parse()
            .unwrap();
        let record = lines
            .next()
            .unwrap()
            .strip_prefix("Distance:")
            .unwrap()
            .split_ascii_whitespace()
            .fold("".to_string(), |acc, s| acc + s)
            .parse()
            .unwrap();

        let win_scenario_count: usize = calc_win_scenario_count(time, record);
        Ok(win_scenario_count.to_string())
    }
}

fn calc_win_scenario_count(time: usize, record: usize) -> usize {
    let mut win_scenario_count: usize = 0;
    for t in 1..=time / 2 {
        if t * (time - t) > record {
            win_scenario_count += 2;
        }
    }
    if time % 2 == 0 {
        win_scenario_count -= 1;
    }
    win_scenario_count
}
