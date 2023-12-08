use crate::utils::{Task, TaskError};
use num::Integer;
use std::collections::HashMap;

pub struct Day8;

impl Task for Day8 {
    #[allow(unused_variables)]
    fn task_part_one(&self, file_content: &str) -> Result<String, TaskError> {
        // return Ok("".to_string());
        let mut lines = file_content.lines();

        let instructions = lines.next().unwrap();
        lines.next();

        let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

        for line in lines {
            let (start, mut end) = line.split_once(" = ").unwrap();
            end = end.strip_prefix('(').unwrap();
            end = end.strip_suffix(')').unwrap();
            let (left, right) = end.split_once(", ").unwrap();
            map.insert(start, (left, right));
        }
        let mut steps = 0;
        let mut loc = "AAA";
        'Outer: loop {
            for c in instructions.chars() {
                loc = match c {
                    'R' => map[loc].1,
                    'L' => map[loc].0,
                    _ => panic!("invalid instruction"),
                };
                steps += 1;
                if loc == "ZZZ" {
                    break 'Outer;
                }
            }
        }

        // dbg!(map);
        Ok(steps.to_string())
    }
    #[allow(unused_variables)]
    fn task_part_two(&self, file_content: &str) -> Result<String, TaskError> {
        let mut lines = file_content.lines();
        let instructions = lines.next().unwrap();
        lines.next();

        let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

        let mut locations: Vec<&str> = Vec::new();

        for line in lines {
            let (start, mut end) = line.split_once(" = ").unwrap();
            if start.ends_with('A') {
                locations.push(start);
            }
            end = end.strip_prefix('(').unwrap();
            end = end.strip_suffix(')').unwrap();
            let (left, right) = end.split_once(", ").unwrap();
            map.insert(start, (left, right));
        }

        let mut steps = Vec::new();
        for loc in locations {
            let mut curr = loc;
            let mut i = 0;
            loop {
                let next = instructions.chars().nth(i % instructions.len()).unwrap();
                curr = match next {
                    'L' => map[curr].0,
                    'R' => map[curr].1,
                    _ => panic!("Invalid instruction"),
                };
                i += 1;
                if curr.ends_with('Z') {
                    break;
                }
            }
            steps.push(i);
        }
        let res = steps.iter().fold(1, |a, b| a.lcm(b));
        Ok(res.to_string())
    }

    fn get_day(&self) -> u32 {
        8
    }
}
