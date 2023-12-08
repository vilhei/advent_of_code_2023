use crate::utils::{Task, TaskError};

pub struct Day1;

impl Task for Day1 {
    fn task_part_one(&self, file_content: &str) -> Result<String, TaskError> {
        let mut sum = 0;
        for line in file_content.lines() {
            let mut first_d = 0;
            let mut last_d = 0;
            for c in line.chars() {
                if c.is_ascii_digit() {
                    first_d = c.to_digit(10).unwrap();
                    break;
                }
            }
            for c in line.chars().rev() {
                if c.is_ascii_digit() {
                    last_d = c.to_digit(10).unwrap();
                    break;
                }
            }
            sum += first_d * 10 + last_d;
        }

        Ok(sum.to_string())
    }

    fn task_part_two(&self, file_content: &str) -> Result<String, TaskError> {
        let nums = [
            (1, "one"),
            (2, "two"),
            (3, "three"),
            (4, "four"),
            (5, "five"),
            (6, "six"),
            (7, "seven"),
            (8, "eight"),
            (9, "nine"),
        ];

        let mut sum = 0;

        for line in file_content.lines() {
            let mut first = usize::MAX;
            let mut first_d = 0;
            let mut last_d = 0;
            let mut last = 0;

            // First check for ascii digits
            for (i, c) in line.chars().enumerate() {
                if c.is_ascii_digit() {
                    first_d = c.to_digit(10).unwrap();
                    first = i;
                    break;
                }
            }
            for (i, c) in line.chars().collect::<Vec<_>>().iter().enumerate().rev() {
                if c.is_ascii_digit() {
                    last_d = c.to_digit(10).unwrap();
                    last = i;
                    break;
                }
            }

            // Secondly check for string representation of digits
            for (idx, n) in nums {
                match line.find(n) {
                    Some(i) if i < first => {
                        first = i;
                        first_d = idx as u32;
                    }
                    _ => (),
                }
                match line.rfind(n) {
                    Some(i) if i > last => {
                        last = i;
                        last_d = idx as u32;
                    }
                    _ => (),
                }
            }

            sum += first_d * 10 + last_d;
        }

        Ok(sum.to_string())
    }

    fn get_day(&self) -> u32 {
        1
    }
}
