use std::env;

use advents::{utils::Task, *};
use main_utils::{create_day, handle_answer};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Give a day to run as argument");
    }

    let day: u64 = args[1].parse().expect("Give a number");
    let file_path = format!("./inputs/day{day}.txt");
    let advent: Box<dyn Task> = create_day(day as u32);

    let part_one_result = advent.task_part_one(&file_path);
    handle_answer(&part_one_result, day);
    let part_two_result = advent.task_part_two(&file_path);
    handle_answer(&part_two_result, day);
}
