use std::env;

use advents::main_utils::process_tasks;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Give a day to run as argument");
    }

    let day: usize = args[1].parse().expect("Give a number");
    let file_path = format!("./inputs/day{day}.txt");
    process_tasks(&file_path, day);
}
