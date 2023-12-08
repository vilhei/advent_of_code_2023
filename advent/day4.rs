use std::collections::HashSet;

use crate::utils::{Task, TaskError};

pub struct Day4;

impl Task for Day4 {
    fn task_part_one(&self, file_content: &str) -> Result<String, TaskError> {
        let mut sum = 0;
        for line in file_content.lines() {
            let res = check_correct_nums(line);
            if res != 0 {
                sum += 1 << (res - 1); // 1 << (res - 1) ==  2^(res-1)
            }
        }
        Ok(sum.to_string())
    }

    fn task_part_two(&self, file_content: &str) -> Result<String, TaskError> {
        let mut cards = vec![0; file_content.lines().count()];
        for (i, line) in file_content.lines().enumerate() {
            let res = check_correct_nums(line);
            cards[i] += 1;
            for j in 1..=res {
                cards[i + j] += cards[i];
            }
        }
        Ok(cards.iter().sum::<usize>().to_string())
    }

    fn get_day(&self) -> u32 {
        4
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
