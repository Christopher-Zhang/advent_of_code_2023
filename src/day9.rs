use std::str;
use std::collections::VecDeque;

pub async fn advent(data: String) -> i32 {
    let mut answer = 0;
    for history in data.lines() {
        let nums: Vec<&str> = history.split(' ').collect();
        let nums: Vec<i32> = nums.iter().map(|num| {
            num.parse::<i32>().unwrap()
        }).collect();

        let cur = solve(nums);
        // println!("Sum next for {:?} is {}", nums, cur);
        answer += cur;
        
    }
    return answer;
}

pub async fn advent_2(data: String) -> i32 {
    let mut answer = 0;
    for history in data.lines() {
        let nums: Vec<&str> = history.split(' ').collect();
        let nums: VecDeque<i32> = nums.iter().map(|num| {
            num.parse::<i32>().unwrap()
        }).collect();

        let cur = solve_2(nums);
        // println!("Sum next for {:?} is {}", nums, cur);
        answer += cur;
        
    }
    return answer;
}

fn get_below(history: &Vec<i32>) -> Vec<i32> {
    let mut ret = Vec::<i32>::new();
    for i in 0 .. history.len() - 1 {
        let cur = history[i];
        let next = history[i+1];
        ret.push(next-cur);
    }
    return ret;
}
fn get_below_2(history: &VecDeque<i32>) -> VecDeque<i32> {
    let mut ret = VecDeque::<i32>::new();
    for i in 0 .. history.len() - 1 {
        let cur = history[i];
        let next = history[i+1];
        ret.push_back(next-cur);
    }
    return ret;
}

fn solve(history: Vec<i32>) -> i32 {
    let mut ret = 0;
    let mut rows: Vec<Vec<i32>> = Vec::new();
    let mut current = history;
    while current.len() > 0 && !current.iter().all(|&e| e == 0) {
        rows.push(current.clone());
        current = get_below(&current);
        // dbg!(&current);
    }

    for row_num in (1..rows.len()).rev() {
        let below = rows[row_num].last().unwrap().clone();
        let current = rows[row_num-1].last().unwrap().clone();
        let next = current + below;
        ret = next;
        rows[row_num-1].push(next);
    }
    return ret;
}
fn solve_2(history: VecDeque<i32>) -> i32 {
    let mut ret = 0;
    let mut rows: VecDeque<VecDeque<i32>> = VecDeque::new();
    let mut current = history;
    while current.len() > 0 && !current.iter().all(|&e| e == 0) {
        rows.push_back(current.clone());
        current = get_below_2(&current);
        // dbg!(&current);
    }

    for row_num in (1..rows.len()).rev() {
        let below = rows[row_num].front().unwrap().clone();
        let current = rows[row_num-1].front().unwrap().clone();
        let next = current-below;
        ret = next;
        rows[row_num-1].push_front(next);
    }
    return ret;
}

