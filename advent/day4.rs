use std::collections::{HashMap, HashSet, VecDeque};

use crate::utils::{Task, TaskError};

pub struct Day4;

impl Task for Day4 {
    fn task_part_one(&self, file_content: &str) -> Result<String, TaskError> {
        let mut sum = 0;
        for line in file_content.lines() {
            let res = check_correct_nums(line);
            if res != 0 {
                sum += 1 << (res - 1);
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

    let guesses = str_to_num_iter(guesses);
    let winning = str_to_num_iter(winning).collect::<HashSet<usize>>();
    guesses.filter(|e| winning.contains(e)).count()
}

fn str_to_num_iter(s: &str) -> impl Iterator<Item = usize> + '_ {
    s.split_whitespace().map(|s| s.parse::<usize>().unwrap())
}

fn parse_line(line: &str) -> (&str, &str) {
    let mut pos = line.find(':').unwrap();
    let nums = &line[pos + 1..];
    pos = nums.find('|').unwrap();
    let (winning, guesses) = nums.split_at(pos);
    (&guesses[1..], winning)
}
