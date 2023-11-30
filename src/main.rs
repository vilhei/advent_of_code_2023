mod main_utils;

use std::env;

use advents::{utils::Task, *};
use main_utils::handle_answer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Give a day to run as argument");
    }

    let day: u64 = args[1].parse().expect("Give a number");
    let file_path = format!("./inputs/day{day}.txt");
    let advent: Box<dyn Task> = match day {
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
        24 => Box::new(Day23),
        _ => panic!("Not acceptable day argument"),
    };

    let part_one_result = advent.task_part_one(&file_path);
    handle_answer(&part_one_result, day);
    let part_two_result = advent.task_part_two(&file_path);
    handle_answer(&part_two_result, day);
}
