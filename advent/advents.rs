pub mod main_utils;
pub mod utils;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

pub use day1::Day1;
pub use day2::Day2;
pub use day3::Day3;
pub use day4::Day4;
pub use day5::Day5;
pub use day6::Day6;
pub use day7::Day7;
pub use day8::Day8;
pub use day9::Day9;

pub use day10::Day10;
pub use day11::Day11;
pub use day12::Day12;
pub use day13::Day13;
pub use day14::Day14;
pub use day15::Day15;
pub use day16::Day16;
pub use day17::Day17;
pub use day18::Day18;
pub use day19::Day19;
pub use day20::Day20;
pub use day21::Day21;
pub use day22::Day22;
pub use day23::Day23;
pub use day24::Day24;
pub use day25::Day25;

use main_utils::Part;
use util_procs::print_exec_time;
use utils::{Task, TaskError};

pub struct Advent<'a> {
    input: &'a str,
    solver: Box<dyn Task>,
}

impl<'a> Advent<'a> {
    pub fn new(input: &'a str, solver: Box<dyn Task>) -> Self {
        Advent { input, solver }
    }

    #[print_exec_time(Part 1 time : )]
    pub fn process_part1(&self) {
        let res = self.solver.task_part_one(self.input);
        self.handle_answer(&res, Part::One);
    }

    #[print_exec_time(Part 2 time : )]
    pub fn process_part2(&self) {
        let res = self.solver.task_part_two(self.input);
        self.handle_answer(&res, Part::Two);
    }

    fn get_day(&self) -> u32 {
        self.solver.get_day()
    }

    fn handle_answer(&self, res: &Result<String, TaskError>, task_n: Part) {
        if let Ok(answer) = res {
            println!("Day {} part {task_n} answer :\n{answer}", self.get_day());
            return;
        }
        match res.as_ref().unwrap_err() {
            TaskError::InvalidFilePath(reason) => panic!("Invalid file path :\n {reason}"),
            TaskError::NotImplemented(task_n) => panic!("Task {task_n} not implemented"),
        }
    }
}
