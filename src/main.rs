use std::{env, time::Instant};

use advents::{
    main_utils::Part,
    utils::{read_task_input_file, Task, TaskError},
    *,
};
use main_utils::{create_day, handle_answer};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Give a day to run as argument");
    }

    let day: u64 = args[1].parse().expect("Give a number");
    let file_path = format!("./inputs/day{day}.txt");

    match read_task_input_file(&file_path) {
        Ok(file_content) => {
            let advent: Box<dyn Task> = create_day(day as u32);
            let mut start = Instant::now();

            let part_one_result = advent.task_part_one(&file_content);
            let mut duration = start.elapsed();
            handle_answer(&part_one_result, day, Part::One, duration);

            start = Instant::now();
            let part_two_result = advent.task_part_two(&file_content);
            duration = start.elapsed();
            handle_answer(&part_two_result, day, Part::One, duration);
        }
        Err(e) => match e {
            TaskError::InvalidFilePath(reason) => panic!("Invalid file path :\n {reason}"),
            TaskError::NotImplemented(task_n) => panic!("Task {task_n} not implemented"),
        },
    }
}
