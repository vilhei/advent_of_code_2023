use crate::utils::{Task, TaskError};
use rayon::prelude::*;

pub struct Day5;

#[derive(Clone, Copy, PartialEq, PartialOrd, Hash, Eq, Ord)]
struct Mapper {
    source_start: i64,
    source_end: i64,
    offset: i64,
}

impl Task for Day5 {
    #[allow(unused_variables)]
    fn task_part_one(&self, file_content: &str) -> Result<String, TaskError> {
        let mut lines = file_content.split("\n\n");
        let seeds = &lines.nth(0).unwrap()[6..];

        let mut maps: Vec<Vec<Mapper>> = vec![Vec::new(); 7];

        for (map_idx, seed_map) in lines.enumerate() {
            let m = seed_map.lines().skip(1);
            for line in m {
                let mapper = parse_mapper(line);

                maps[map_idx].push(mapper);
            }
        }

        let mut min_loc = i64::MAX;

        for seed in seeds.split_ascii_whitespace() {
            let mut key = seed.parse::<i64>().unwrap();
            key = run_pipeline_for_seed(&maps, key);
            min_loc = std::cmp::min(min_loc, key);
        }

        Ok(min_loc.to_string())
    }

    #[allow(unused_variables)]
    fn task_part_two(&self, file_content: &str) -> Result<String, TaskError> {
        let mut lines = file_content.split("\n\n");
        let seeds = &lines.nth(0).unwrap()[6..];

        let mut maps: Vec<Vec<Mapper>> = vec![Vec::new(); 7];

        for (map_idx, seed_map) in lines.enumerate() {
            let m = seed_map.lines().skip(1);
            for line in m {
                let mapper = parse_mapper(line);
                maps[map_idx].push(mapper);
            }
        }

        let seeds = seeds
            .split_ascii_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
            .chunks(2)
            .flat_map(|a| [a[0], a[1] / 2, a[0] + a[1] / 2, a[1] / 2])
            .collect::<Vec<i64>>();

        let res = seeds
            .chunks(2)
            .par_bridge()
            .map(|a| {
                let s1 = a[0];
                let s2 = a[1];
                let seed_n = s1 + s2 - s1;
                (s1..s1 + s2)
                    .into_par_iter()
                    .map(|seed| run_pipeline_for_seed(&maps, seed))
                    .min()
            })
            .min()
            .unwrap()
            .unwrap();

        Ok(res.to_string())
    }

    fn get_day(&self) -> u32 {
        5
    }
}

fn run_pipeline_for_seed(pipeline: &Vec<Vec<Mapper>>, mut seed: i64) -> i64 {
    for map in pipeline {
        for map_range in map {
            if seed >= map_range.source_start && seed <= map_range.source_end {
                seed += map_range.offset;
                break;
            }
        }
    }
    seed
}

fn parse_mapper(line: &str) -> Mapper {
    let mut ranges = line.split_ascii_whitespace();
    let dest_start = ranges.next().unwrap().parse::<i64>().unwrap();
    let source_start = ranges.next().unwrap().parse::<i64>().unwrap();
    let range = ranges.next().unwrap().parse::<i64>().unwrap();

    Mapper {
        source_start,
        source_end: source_start + range - 1,
        offset: dest_start - source_start,
    }
}
