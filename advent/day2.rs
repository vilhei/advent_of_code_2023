use std::collections::HashMap;

use crate::utils::{read_task_input_file, Task, TaskError};
use nom::{
    bytes::complete::{is_a, is_not, take_until1, take_while1},
    character::complete::{char, digit1},
    multi::separated_list1,
    sequence::pair,
    IResult,
};

pub struct Day2;

fn not_a_digit(c: char) -> bool {
    !c.is_ascii_digit()
}

fn parse_id(input: &str) -> IResult<&str, u32> {
    let (input, game) = take_until1(":")(input)?;
    let (game, _) = take_while1(not_a_digit)(game)?;
    let id = game.trim().parse::<u32>().unwrap();
    let (input, _) = char(':')(input)?;
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
    fn task_part_one(&self, input_file: &str) -> Result<String, TaskError> {
        // To suppress warning about unused file_content in generated files
        #[allow(unused_variables)]
        let mut file_content = read_task_input_file(input_file)?;
        file_content = file_content.replace(' ', "");
        let max_red = 12;
        let max_green = 13;
        let max_blue = 14;

        let max_table = HashMap::from([
            ("green".to_string(), max_green),
            ("blue".to_string(), max_blue),
            ("red".to_string(), max_red),
        ]);
        let mut sum = 0;
        'outer: for line in file_content.lines() {
            let (input, id) = parse_id(line).unwrap();
            let (_, sets) = parse_line(input).unwrap();
            for set in sets {
                for (color, n) in set {
                    if n > max_table[&color] {
                        continue 'outer;
                    }
                }
            }
            sum += id;
        }

        Ok(sum.to_string())
    }

    fn task_part_two(&self, input_file: &str) -> Result<String, TaskError> {
        // To suppress warning about unused file_content in generated files
        #[allow(unused_variables)]
        let mut file_content = read_task_input_file(input_file)?;
        file_content = file_content.replace(' ', "");

        let mut sum = 0;
        for line in file_content.lines() {
            let (input, _) = parse_id(line).unwrap();
            let (_, game) = parse_line(input).unwrap();

            let mut min_red = 0;
            let mut min_green = 0;
            let mut min_blue = 0;

            for set in game {
                for (color, n) in set {
                    match color.as_str() {
                        "red" if n > min_red => min_red = n,
                        "green" if n > min_green => min_green = n,
                        "blue" if n > min_blue => min_blue = n,
                        _ => (),
                    }
                }
            }
            sum += min_red * min_green * min_blue;
        }

        Ok(sum.to_string())
    }
}
