use std::{str, collections::HashSet};

use itertools::Itertools;

pub async fn advent(data: String) -> usize {
    let mut answer = 0;
    for line in data.lines() {
        let groups: Vec<&str> = line.split(' ').collect();
        let mut row: Vec<char> = Vec::new();
        let counts: Vec<usize> = groups[1].split(',').collect_vec().iter().map(|c| c.parse::<usize>().unwrap()).collect_vec();
        for c in groups[0].chars() {
            row.push(c);
        }
        let mut seen: HashSet<String> = HashSet::new();
        answer += solve(&mut row, &counts, 0, &mut seen);
        // println!("{:?}? : {} for counts: {:?}", row.clone(), is_valid(&mut row, &counts), counts.clone());

    }
    return answer;
}
fn solve(row: &mut Vec<char>, counts: &Vec<usize>, index: usize, seen: &mut HashSet<String>) -> usize {
    if index >= row.len() {
        if is_valid(row, counts) {
            // println!("{:?}? : {} for counts: {:?}", row, is_valid(row, counts), counts);
            return 1;
        }
        else {
            return 0;
        }
    }
    let mut ways = 0;
    if row[index] == '?' {
        let s: String = row.iter().collect();
        if seen.contains(&s) {
            return 0;
        }
        else {
            seen.insert(s);
        }
        row[index] = '#';
        ways += solve(row, counts, index + 1, seen);
        row[index] = '.';
        ways += solve(row, counts, index + 1, seen);
        row[index] = '?';
    }
    else {
        ways += solve(row, counts, index + 1, seen);
    }

    return ways;
}

fn is_valid(row: &Vec<char>, counts: &Vec<usize>) -> bool {

    let mut count_index = 0;
    let mut count = counts[count_index];
    let mut row_index = 0;
    while count_index < counts.len() && row_index < row.len() {
        if row[row_index] == '#' {
            if row_index + count <= row.len() && row[row_index..row_index + count].iter().all(|c| c == &'#') {
                count_index += 1;
                row_index += count;
                if row_index < row.len() && row[row_index] == '#' {
                    return false;
                }
                if count_index >= counts.len() {
                    break;
                }
                count = counts[count_index];
            }
            else {
                return false;
            }
        }
        row_index += 1;
    }
    if row_index < row.len() && row[row_index..row.len()].iter().any(|c| c == &'#') {
        return false;
    }
    return count_index >= counts.len();
}

pub async fn advent_2(data: String) -> usize {
    let mut answer = 0;
    for line in data.lines() {
        let groups: Vec<&str> = line.split(' ').collect();
        let mut row: Vec<char> = Vec::new();
        let mut counts: Vec<usize> = groups[1].split(',').collect_vec().iter().map(|c| c.parse::<usize>().unwrap()).collect_vec();
        counts = [&counts[..],&counts[..],&counts[..],&counts[..],&counts[..]].concat();
        for c in groups[0].chars() {
            row.push(c);
        }
        row = [&row[..], &['?'], &row[..], &['?'], &row[..], &['?'], &row[..], &['?'], &row[..]].concat();
        // println!("{:?}", &row);
        // println!("{:?}", &counts);
        let mut seen: HashSet<String> = HashSet::new();
        answer += solve(&mut row, &counts, 0, &mut seen);
        // println!("{:?}? : {} for counts: {:?}", row.clone(), is_valid(&mut row, &counts), counts.clone());
        println!("answer = {}", answer);

    }
    return answer;
}