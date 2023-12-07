use std::fmt::Display;

use util_procs::print_exec_time;

// use advents::utils::{Task, TaskError};
use crate::{
    utils::{read_task_input_file, Task, TaskError},
    *,
};

pub fn create_day(day: u32) -> Box<dyn Task> {
    match day {
        1 => Box::new(Day1),
        2 => Box::new(Day2),
        3 => Box::new(Day3),
        4 => Box::new(Day4),
        5 => Box::new(Day5),
        6 => Box::new(Day6),
        7 => Box::new(Day7),
        8 => Box::new(Day8),
        9 => Box::new(Day9),
        10 => Box::new(Day10),
        11 => Box::new(Day11),
        12 => Box::new(Day12),
        13 => Box::new(Day13),
        14 => Box::new(Day14),
        15 => Box::new(Day15),
        16 => Box::new(Day16),
        17 => Box::new(Day17),
        18 => Box::new(Day18),
        19 => Box::new(Day19),
        20 => Box::new(Day20),
        21 => Box::new(Day21),
        22 => Box::new(Day22),
        23 => Box::new(Day23),
        24 => Box::new(Day24),
        25 => Box::new(Day25),
        _ => panic!("Not acceptable day argument"),
    }
}

pub enum Part {
    One,
    Two,
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Part::One => "one",
            Part::Two => "two",
        };
        write!(f, "{}", s)
    }
}

pub fn process_tasks(file_path: &str, day: usize) {
    match read_task_input_file(file_path) {
        Ok(file_content) => {
            let advent: Box<dyn Task> = create_day(day as u32);
            run_task(&advent, &file_content, day, Part::One);
            run_task(&advent, &file_content, day, Part::Two);
        }
        Err(e) => match e {
            TaskError::InvalidFilePath(reason) => panic!("Invalid file path :\n {reason}"),
            TaskError::NotImplemented(task_n) => panic!("Task {task_n} not implemented"),
        },
    }
}

#[print_exec_time(Exec time : )]
#[allow(clippy::borrowed_box)]
pub fn run_task(advent: &Box<dyn Task>, file_content: &str, day: usize, task_n: Part) {
    let task_result = match task_n {
        Part::One => advent.task_part_one(file_content),
        Part::Two => advent.task_part_two(file_content),
    };
    handle_answer(&task_result, day, task_n);
}

pub fn handle_answer(res: &Result<String, TaskError>, day: usize, task_n: Part) {
    if let Ok(answer) = res {
        println!("Day {day} part {task_n} answer :\n{answer}",);
        return;
    }
    match res.as_ref().unwrap_err() {
        TaskError::InvalidFilePath(reason) => panic!("Invalid file path :\n {reason}"),
        TaskError::NotImplemented(task_n) => panic!("Task {task_n} not implemented"),
    }
}
