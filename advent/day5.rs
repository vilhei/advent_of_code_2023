use std::{
    sync::{Arc, Mutex},
    thread::{self},
};

use crate::utils::{Task, TaskError};

pub struct Day5;

#[derive(Clone, Copy, PartialEq, PartialOrd, Hash, Eq, Ord)]
struct M {
    source_start: i64,
    source_end: i64,
    offset: i64,
}

impl Task for Day5 {
    #[allow(unused_variables)]
    fn task_part_one(&self, file_content: &str) -> Result<String, TaskError> {
        let mut lines = file_content.split("\n\n");
        let seeds = &lines.nth(0).unwrap()[6..];

        let mut maps: Vec<Vec<M>> = vec![Vec::new(); 7];

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

        let mut maps: Vec<Vec<M>> = vec![Vec::new(); 7];

        for (map_idx, seed_map) in lines.enumerate() {
            let m = seed_map.lines().skip(1);
            for line in m {
                let mapper = parse_mapper(line);
                maps[map_idx].push(mapper);
            }
        }

        let min_loc = Arc::new(Mutex::new(i64::MAX));
        let seeds = seeds
            .split_ascii_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
            .chunks(2)
            .flat_map(move |a| [a[0], a[1] / 2, a[0] + a[1] / 2, a[1] / 2])
            .collect::<Vec<i64>>();
        // println!("{seeds:?}");
        let mut handles = Vec::new();
        for a in seeds.chunks(2) {
            let s1 = a[0];
            let s2 = a[1];
            let pipeline_map = maps.clone();
            let curr_min = Arc::clone(&min_loc);
            let handle = thread::spawn(move || {
                let mut locations = Vec::new();
                for seed in s1..s1 + s2 {
                    let mut key = seed;
                    key = run_pipeline_for_seed(&pipeline_map, key);
                    locations.push(key);
                }
                let min = locations.iter().min().unwrap();
                let mut min_loc = curr_min.lock().unwrap();
                *min_loc = std::cmp::min(*min_loc, *min);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
        let min_loc = min_loc.lock().unwrap();
        Ok(min_loc.to_string())
    }
}

fn run_pipeline_for_seed(pipeline: &Vec<Vec<M>>, mut seed: i64) -> i64 {
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

fn parse_mapper(line: &str) -> M {
    let mut ranges = line.split_ascii_whitespace();
    let dest_start = ranges.next().unwrap().parse::<i64>().unwrap();
    let source_start = ranges.next().unwrap().parse::<i64>().unwrap();
    let range = ranges.next().unwrap().parse::<i64>().unwrap();

    M {
        source_start,
        source_end: source_start + range - 1,
        offset: dest_start - source_start,
    }
}
