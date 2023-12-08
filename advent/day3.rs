use std::collections::HashMap;

use crate::utils::{Task, TaskError};

pub struct Day3;

impl Task for Day3 {
    fn task_part_one(&self, file_content: &str) -> Result<String, TaskError> {
        let lines: Vec<&str> = file_content.lines().collect();
        let mut sum = 0;
        for (line_idx, line) in lines.iter().enumerate() {
            let mut s_idx = None;
            let mut e_idx = 0;
            let mut digits = Vec::new();
            let mut prev_is_digit = false;
            for (char_idx, c) in line.chars().enumerate() {
                if c.is_ascii_digit() {
                    if s_idx.is_none() {
                        s_idx = Some(char_idx);
                    }
                    digits.push(c.to_digit(10).unwrap());
                    e_idx = char_idx;
                    prev_is_digit = true;
                }
                if prev_is_digit && (char_idx == line.len() - 1 || !c.is_ascii_digit()) {
                    let num = check_if_valid_num(
                        line_idx,
                        &lines,
                        s_idx.unwrap(),
                        e_idx,
                        line,
                        &mut digits,
                    );

                    if let Some(n) = num {
                        sum += n;
                    }

                    digits.clear();
                    s_idx = None;
                    prev_is_digit = false;
                }
            }
        }
        Ok(sum.to_string())
    }

    fn task_part_two(&self, file_content: &str) -> Result<String, TaskError> {
        let lines: Vec<&str> = file_content.lines().collect();

        let mut nums: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
        for (line_idx, line) in lines.iter().enumerate() {
            let mut s_idx = None;
            let mut e_idx = 0;
            let mut digits = Vec::new();
            let mut prev_is_digit = false;
            for (char_idx, c) in line.chars().enumerate() {
                if c.is_ascii_digit() {
                    if s_idx.is_none() {
                        s_idx = Some(char_idx);
                    }

                    digits.push(c.to_digit(10).unwrap());
                    e_idx = char_idx;
                    prev_is_digit = true;
                }

                if prev_is_digit && (char_idx == line.len() - 1 || !c.is_ascii_digit()) {
                    let num = check_if_next_to_star(
                        line_idx,
                        &lines,
                        s_idx.unwrap(),
                        e_idx,
                        line,
                        &mut digits,
                    );

                    if let Some((key, n)) = num {
                        nums.entry(key).or_default().push(n);
                    }

                    s_idx = None;
                    prev_is_digit = false;
                    digits.clear();
                }
            }
        }
        let sum = nums
            .iter()
            .filter(|(_, v)| v.len() == 2)
            .fold(0, |p, (_, v)| p + v[0] * v[1]);
        Ok(sum.to_string())
    }

    fn get_day(&self) -> u32 {
        3
    }
}

fn check_if_valid_num(
    line_idx: usize,
    lines: &[&str],
    s_idx: usize,
    e_idx: usize,
    line: &&str,
    digits: &mut [u32],
) -> Option<u32> {
    let validation_fn = |c: &char| c != &'.' && !c.is_ascii_digit() && *c != '\n';

    let valid_number = check_adjacent_chars(line_idx, lines, s_idx, e_idx, line, validation_fn);
    if valid_number.is_some() {
        let num = digits_to_num(digits);
        return Some(num);
    }
    None
}

fn check_if_next_to_star(
    line_idx: usize,
    lines: &[&str],
    s_idx: usize,
    e_idx: usize,
    line: &str,
    digits: &mut [u32],
) -> Option<((usize, usize), usize)> {
    let validation_fn = |c: &char| c == &'*';

    let star_adjacent = check_adjacent_chars(line_idx, lines, s_idx, e_idx, line, validation_fn);

    if let Some(position) = star_adjacent {
        let num = digits_to_num(digits);
        return Some((position, num as usize));
    }
    None
}

fn check_adjacent_chars<F>(
    line_idx: usize,
    lines: &[&str],
    s_idx: usize,
    e_idx: usize,
    line: &str,
    validation_fn: F,
) -> Option<(usize, usize)>
where
    F: Fn(&char) -> bool,
{
    let mut checkable_lines_idx = Vec::new();

    checkable_lines_idx.push(line_idx);

    if line_idx > 1 {
        checkable_lines_idx.push(line_idx - 1);
    };

    if line_idx < lines.len() - 1 {
        checkable_lines_idx.push(line_idx + 1);
    };

    let s_idx = if s_idx > 1 { s_idx - 1 } else { s_idx };
    let e_idx = if e_idx < line.len() { e_idx + 1 } else { e_idx };

    let mut valid_number = false;
    let mut position = (0, 0);
    'outer: for li in checkable_lines_idx {
        let chars: Vec<char> = lines[li].chars().collect();
        for (i, c) in chars.iter().enumerate().take(e_idx + 1).skip(s_idx) {
            if validation_fn(c) {
                valid_number = true;
                position = (li, i);
                break 'outer;
            }
        }
    }

    match valid_number {
        true => Some(position),
        false => None,
    }
}

fn digits_to_num(digits: &mut [u32]) -> u32 {
    let mut mult = 1;
    let mut num = 0;
    digits.reverse();
    for d in digits {
        num += *d * mult;
        mult *= 10;
    }
    num
}
