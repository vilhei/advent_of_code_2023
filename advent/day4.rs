use std::collections::{HashMap, HashSet, VecDeque};

use crate::utils::{Task, TaskError};

pub struct Day4;

impl Task for Day4 {
    fn task_part_one(&self, file_content: &str) -> Result<String, TaskError> {
        let mut sum = 0;
        for line in file_content.lines() {
            let res = check_correct_nums(line);
            if res != 0 {
                let num = 2usize.pow(res as u32 - 1);
                sum += num;
            }
        }
        Ok(sum.to_string())
    }

    fn task_part_two(&self, file_content: &str) -> Result<String, TaskError> {
        let lines = file_content.lines().collect::<Vec<_>>();

        let mut queue: VecDeque<usize> = (0..lines.len()).collect();
        let mut winnings_cache: HashMap<usize, usize> = HashMap::with_capacity(lines.len());

        let mut sum = 0;
        while let Some(line_n) = queue.pop_front() {
            sum += 1;

            let line_win_amount = *winnings_cache
                .entry(line_n)
                .or_insert_with(|| check_correct_nums(lines[line_n]));

            for r in 1..=line_win_amount {
                queue.push_back(line_n + r);
            }
        }

        Ok(sum.to_string())
    }
}

fn check_correct_nums(line: &str) -> usize {
    let (guesses, winning) = parse_line(line);

    let guesses: HashSet<_> = guesses
        .split(' ')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let winning: HashSet<usize> = winning
        .split(' ')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let res = guesses.intersection(&winning).collect::<Vec<_>>().len();
    res
}

fn parse_line(line: &str) -> (String, String) {
    let pos = line.find(':').unwrap();
    let nums = &line[pos + 1..];
    let pos = nums.find('|').unwrap();
    let (winning, guesses) = nums.split_at(pos);
    let guesses = guesses[1..].trim().replace("  ", " ");
    let winning = winning.trim().replace("  ", " ");

    (guesses, winning)
}
