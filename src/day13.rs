use std::str;
use str_distance::{Levenshtein, str_distance, DistanceValue};

pub async fn advent(data: String) -> usize {
    let mut answer = 0;
    for pattern in data.split("\n\n") {
        let current = solve(pattern);
        answer += current;
    }
    return answer;
}

fn solve (data: &str) -> usize {
    let mut rows: Vec<String> = Vec::new();
    let mut cols: Vec<String> = Vec::new();
    let mut first = true;
    for line in data.lines() {
        rows.push(line.to_string());
        for (i,c) in line.chars().enumerate() {
            if first {
                cols.push(c.to_string());
            }
            else {
                cols[i].push(c);
            }
        }
        first = false;
    }

    for (i, _) in rows.iter().enumerate() {
        if is_reflection(&rows, i) {
            return 100 * (i+1);
        }
    }
    for (i, _) in cols.iter().enumerate() {
        if is_reflection(&cols, i) {
            return i + 1;
        }
    }

    panic!("no answer for {data}");
}

fn is_reflection(strings: &Vec<String>, index: usize) -> bool {
    let mut left: i64 = index as i64;
    let mut right: i64 = index as i64 + 1;

    if index + 1 >= strings.len() {
        return false;
    }

    while left >= 0 && right < strings.len() as i64 {
        if strings[left as usize] != strings[right as usize] {
            return false;
        }
        left -= 1;
        right += 1;
    }
    return true;
}

pub async fn advent_2(data: String) -> usize {
    let mut answer = 0;
    for pattern in data.split("\n\n") {
        let current = solve2(pattern);
        answer += current;
    }
    return answer;
}

fn solve2(data: &str) -> usize {
    dbg!(str_distance("abcassdfadfas", "abdasdfasfadf", Levenshtein::with_max_distance(2)));
    let mut rows: Vec<String> = Vec::new();
    let mut cols: Vec<String> = Vec::new();
    let mut first = true;
    for line in data.lines() {
        rows.push(line.to_string());
        for (i,c) in line.chars().enumerate() {
            if first {
                cols.push(c.to_string());
            }
            else {
                cols[i].push(c);
            }
        }
        first = false;
    }

    for (i, _) in rows.iter().enumerate() {
        if is_one_off(&rows, i) {
            return 100 * (i+1);
        }
    }
    for (i, _) in cols.iter().enumerate() {
        if is_one_off(&cols, i) {
            return i + 1;
        }
    }

    panic!("no answer for {data}");
}

fn is_one_off(strings: &Vec<String>, index: usize) -> bool {
    let mut left: i64 = index as i64;
    let mut right: i64 = index as i64 + 1;
    let mut one_off = false;
    if index + 1 >= strings.len() {
        return false;
    }
    while left >= 0 && right < strings.len() as i64 {
        let left_str = strings[left as usize].as_str();
        let right_str = strings[right as usize].as_str();
        let distance = str_distance(left_str, right_str, Levenshtein::with_max_distance(2));
        if distance == DistanceValue::Exact(1) {
            if one_off {
                return false;
            }
            one_off = true;
        }
        else if left_str != right_str {
            return false;
        }
        left -= 1;
        right += 1;
    }
    return one_off;
}