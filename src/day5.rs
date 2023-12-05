use std::collections::VecDeque;
use std::str;
use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug)]
struct Ranges {
    dest: (u64, u64),
    source: (u64, u64)
}

pub async fn advent(data: String) -> u64 {
    let mut groups: VecDeque<&str> = data.split("\n\n").collect();
    let mut seeds: VecDeque<&str> = groups.pop_front().unwrap().split(' ').collect();
    seeds.pop_front();
    let seeds: Vec<u64> = seeds.iter().map(|v| {
        v.parse::<u64>().unwrap_or(0)
    }).collect();
    let mut stages: Vec<Vec<Ranges>> = Vec::new();
    for group in groups {
        let mut stage = Vec::<Ranges>::new();
        let mut group: VecDeque<&str> = group.split('\n').collect();
        group.pop_front();

        for row in group {
            let row: VecDeque<u64> = row.split(' ').map(|v| {
                v.parse::<u64>().unwrap_or(0)
            }).collect();

            let dest = (row[0], row[0] + row[2]);
            let source = (row[1], row[1] + row[2]);
            let range = Ranges {
                dest, source
            };
            stage.push(range);
        }
        stages.push(stage);
    }
    let mut min = u64::MAX;
    for seed in seeds {
        let location = find_location(seed, &stages);
        min = std::cmp::min(min, location);
    }
    min
}

pub async fn advent_2(data: String) -> u64 {
    let mut groups: VecDeque<&str> = data.split("\n\n").collect();
    let mut seeds: VecDeque<&str> = groups.pop_front().unwrap().split(' ').collect();
    seeds.pop_front();
    let seeds: Vec<u64> = seeds.iter().map(|v| {
        v.parse::<u64>().unwrap_or(0)
    }).collect();
    let mut stages: Vec<Vec<Ranges>> = Vec::new();
    for group in groups {
        let mut stage = Vec::<Ranges>::new();
        let mut group: VecDeque<&str> = group.split('\n').collect();
        group.pop_front();

        for row in group {
            let row: VecDeque<u64> = row.split(' ').map(|v| {
                v.parse::<u64>().unwrap_or(0)
            }).collect();

            let dest = (row[0], row[0] + row[2]);
            let source = (row[1], row[1] + row[2]);
            let range = Ranges {
                dest, source
            };

            stage.push(range);
        }
        stages.push(stage);
    }
    // dbg!(&stages);
    let seeds: Vec<(&u64, &u64)> = seeds.iter().tuples().collect();
    let min: u64 = seeds.par_iter().map(|(first, last)| {
        let first = first.clone().clone();
        let last = last.clone().clone();
        let reduced = (first..first+last).into_par_iter().map(|seed| {
            find_location(seed, &stages)
        }).reduce(|| u64::MAX,|a, b| std::cmp::min(a,b));
        reduced
    }).reduce(|| u64::MAX, |a,b| std::cmp::min(a,b));
    min
}

fn in_range(num: u64, range: (u64, u64)) -> bool {
    num >= range.0 && num < range.1
}

fn find_location(seed: u64, stages: &Vec<Vec<Ranges>>) -> u64 {
    let mut cur = seed.clone();
    for stage in stages {
        // do a step
        for range in stage {
            let dest = range.dest;
            let source = range.source;
            if in_range(cur, source) {
                cur = dest.0 + (cur - source.0);
                break;
            }
        }
    }
    return cur;
}