// use advents::utils::{Task, TaskError};
use crate::{
    utils::{Task, TaskError},
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

pub fn handle_answer(res: &Result<String, TaskError>, day: u64) {
    if let Ok(answer) = res {
        println!("Day {day} answer :\n{answer}");
        return;
    }
    match res.as_ref().unwrap_err() {
        TaskError::InvalidFilePath(reason) => panic!("Invalid file path :\n {reason}"),
        TaskError::NotImplemented(task_n) => panic!("Task {task_n} not implemented"),
    }
}
