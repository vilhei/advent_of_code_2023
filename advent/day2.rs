use std::collections::HashMap;

use crate::utils::{Task, TaskError};
use nom::{
    bytes::complete::{is_a, is_not, tag},
    character::complete::{char, digit1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{delimited, pair},
    IResult,
};

pub struct Day2;

fn parse_id(input: &str) -> IResult<&str, u32> {
    let parse_num = map_res(digit1, |s: &str| s.parse::<u32>());
    let (input, id) = delimited(tag("Game"), parse_num, char(':'))(input)?;
    Ok((input, id))
}

fn parse_line(input: &str) -> IResult<&str, Vec<Vec<(String, u32)>>> {
    let (input, sets) = separated_list1(is_a(";"), is_not(";"))(input)?;
    let mut game_list = Vec::new();
    for set in sets {
        let (_, res) = separated_list1(char(','), pair(digit1, is_not(",")))(set)?;
        let mut temp_vec = Vec::new();
        for (n, color) in res {
            temp_vec.push((color.to_owned(), n.parse::<u32>().unwrap()));
        }
        game_list.push(temp_vec);
    }
    Ok((input, game_list))
}

impl Task for Day2 {
    fn task_part_one(&self, file_content: &str) -> Result<String, TaskError> {
        let file_content = file_content.replace(' ', "");

        let max_table = HashMap::from([
            ("green".to_string(), 13),
            ("blue".to_string(), 14),
            ("red".to_string(), 12),
        ]);
        let mut sum = 0;
        for line in file_content.lines() {
            let (input, id) = parse_id(line).unwrap();
            let (_, game) = parse_line(input).unwrap();
            let flag = game
                .iter()
                .flatten()
                .any(|(color, n)| n > &max_table[color]);
            if !flag {
                sum += id;
            }
        }

        Ok(sum.to_string())
    }

    fn task_part_two(&self, file_content: &str) -> Result<String, TaskError> {
        let file_content = file_content.replace(' ', "");

        let mut sum = 0;
        for line in file_content.lines() {
            let (input, _) = parse_id(line).unwrap();
            let (_, game) = parse_line(input).unwrap();

            let mut min_red = 0;
            let mut min_green = 0;
            let mut min_blue = 0;

            game.iter()
                .flatten()
                .for_each(|(color, n)| match color.as_str() {
                    "red" if n > &min_red => min_red = *n,
                    "green" if n > &min_green => min_green = *n,
                    "blue" if n > &min_blue => min_blue = *n,
                    _ => (),
                });

            sum += min_red * min_green * min_blue;
        }

        Ok(sum.to_string())
    }
}
