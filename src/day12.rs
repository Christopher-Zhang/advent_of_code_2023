use std::{str, collections::{HashMap}};
use anyhow::{Result, anyhow};
use itertools::Itertools;

pub async fn advent(data: String) -> usize {
    let mut answer = 0;
    let mut seen: Cache= HashMap::new();
    for line in data.lines() {
        let groups: Vec<&str> = line.split(' ').collect();
        let mut row: Vec<char> = Vec::new();
        let counts: Vec<usize> = groups[1].split(',').collect_vec().iter().map(|c| c.parse::<usize>().unwrap()).collect_vec();
        let mut remaining = 0;
        for c in groups[0].chars() {
            row.push(c);
            if c != '.' {
                remaining += 1;
            }
        }
        // let mut seen: HashSet<String> = HashSet::new();
        // answer += solve(&mut row, &counts, 0, &mut seen);
        // for i in 0..row.len() {
        //     println!("Block size for {:?} is {:?}", row.clone(), get_block_size(&mut row, i));
        // }
        // let mut groups: Vec<usize> = Vec::new();
        // answer += solve(&mut row, &counts, 0, 0);
        let current = solve2(&row, &counts, remaining, &mut seen);
        answer += current;//, &mut groups, &mut seen);
        println!("answer = {}", current);
        // println!("row {:?} now has answer {}", row, answer);
        // println!("{:?}? : {} for counts: {:?}", row.clone(), is_valid(&mut row, &counts), counts.clone());
    }
    return answer;
}

// fn solve(row: &mut Vec<char>, counts: &Vec<usize>, row_index: usize, count_index: usize) -> usize {//, groups: &mut Vec<usize>) -> usize {
//     if row_index >= row.len() {
//         if count_index >= counts.len() {
//             return 1;
//         }
//         return 0;
//     }
//     if count_index >= counts.len() {
//         if row[row_index..].iter().any(|c| c == &'#') {
//             return 0;
//         }
//         return 1;
//     }
//     if row[row_index] == '.' {
//         return solve(row, counts, row_index+1, count_index);
//     }
//     let current_count = counts[count_index];
//     let block_size = get_block_size(row, row_index);
//     if block_size.0 < current_count {
//         if block_size.1 > 0 {
//             return 0;
//         }
//         else {
//             return solve(row, counts, row_index + block_size.0, count_index);
//         }
//     }
//     else {
//         if row_index + current_count >= row.len() || row[row_index + current_count] != '#' {
//             // groups.push(row_index);
//             let ret = solve(row, counts, row_index + current_count + 1, count_index + 1);
//             // groups.pop();
//             if row[row_index] == '#' {
//                 return ret;
//             }
//             return ret + solve(row, counts, row_index + 1, count_index);
//         }
//         else {
//             return solve(row, counts, row_index + 1, count_index);
//         }
//     }
// }

// fn is_valid(row: &Vec<char>, counts: &Vec<usize>) -> bool {
//     let mut count_index = 0;
//     let mut current_count = counts[count_index];
//     let mut i = 0;
//     let mut done = false;
//     while i < row.len() {
//         let mut group_size = 0;
//         while i < row.len() && row[i] == '#' {
//             group_size += 1;
//             i += 1;
//         }
//         if group_size > 0 {
//             if done {
//                 println!("done for {:?} {:?}", row, counts);
//                 return false;
//             }
//             if group_size == current_count {
//                 count_index += 1;
//                 if count_index < counts.len() {
//                     current_count = counts[count_index];
//                 }
//                 else {
//                     done = true;
//                 }
//             }
//             else {
//                 return false;
//             }
//         }
//         i += 1;
//     }

//     return count_index >= counts.len();
// }

pub async fn advent_2(data: String) -> usize {
    let mut answer = 0;
    for line in data.lines() {
        let mut seen: Cache = HashMap::new();
        let groups: Vec<&str> = line.split(' ').collect();
        let mut row: Vec<char> = Vec::new();
        let mut counts: Vec<usize> = groups[1].split(',').collect_vec().iter().map(|c| c.parse::<usize>().unwrap()).collect_vec();
        counts = [&counts[..],&counts[..],&counts[..],&counts[..],&counts[..]].concat();
        let mut remaining = 0;
        for c in groups[0].chars() {
            row.push(c);
        }
        row = [&row[..], &['?'], &row[..], &['?'], &row[..], &['?'], &row[..], &['?'], &row[..]].concat();
        for c in &row {
            if c != &'.' {
                remaining += 1;
            }
        }
        // println!("{:?}", row.iter().collect::<String>());
        // println!("{:?}", &counts);
        // let mut seen: HashSet<String> = HashSet::new();
        // let mut groups: Vec<usize> = Vec::new();
        let current = solve2(&row, &counts, remaining, &mut seen);
        answer += current;//, &mut groups, &mut seen);
        // println!("{:?}? : {} for counts: {:?}", row.clone(), is_valid(&mut row, &counts), counts.clone());
        // println!("answer = {}", current);

    }
    return answer;
}

// fn solve2(row: &mut Vec<char>, counts: &Vec<usize>, row_index: usize, count_index: usize, remaining: i64) -> usize {//, groups: &mut Vec<usize>, seen: &mut HashSet<String>) -> usize {
//     // println!("rowindex {}, count {}", row_index, count_index);
//     if row_index >= row.len() {
//         if count_index >= counts.len() {
//             // println!("g{:?}", groups);
//             return 1;
//         }
//         // println!("g{:?}", groups);
//         return 0;
//     }
//     if count_index >= counts.len() {
//         // println!("g{:?}", groups);
//         if row[row_index..].iter().any(|c| c == &'#') {
//             // println!("overflow");
//             return 0;
//         }
//         // println!("g{:?}", groups);
//         return 1;
//     }

//     let sum = counts[count_index..].iter().sum::<usize>() as i64;
//     if sum > remaining {
//         println!("sum = {sum}, remaining = {remaining}");
//         dbg!(counts);
//         return 0;
//     }

//     if row[row_index] == '.' {
//         let next_block = get_next_block(row, row_index+1);
//         match next_block {
//             Ok(i) => return solve2(row, counts, i, count_index, remaining),
//             Err(_) => return 0
//         };
//         // return solve2(row, counts, row_index+1, count_index, groups, seen);
//     }
//     let current_count = counts[count_index];
//     let block_size = get_block_size(row, row_index);
//     // println!("block size: {:?}", block_size);
//     if block_size.0 < current_count {
//         if block_size.1 > 0 {
//             return 0;
//         }
//         else {
//             return solve2(row, counts, row_index + block_size.0 + 1, count_index, remaining - (block_size.0 as i64));//, groups, seen);
//         }
//     }
//     else {
//         let next_block = get_next_block(row, row_index+1);
//         let next_block = match next_block {
//             Ok(i) => i,
//             Err(_) => row.len()
//         };
//         if row_index + current_count >= row.len() || row[row_index + current_count] != '#' {
//             // groups.push(row_index);
//             let ret: usize;
//             // let st = groups.iter().join(",");
//             // println!("{}", st);
//             // if !seen.contains(&st) {
//             //     seen.insert(st);
//             //     ret = solve2(row, counts, row_index + current_count + 1, count_index + 1);//, groups, seen);
//             // }
//             // else {
//             //     println!("Seen {}", st);
//             //     ret = 0;
//             // }
//             // groups.pop();
//             let ret = solve2(row, counts, row_index + current_count + 1, count_index + 1, remaining - (current_count as i64));
//             if row[row_index] == '#' {
//                 return ret;
//             }
//             return ret + solve2(row, counts, next_block, count_index, remaining);//, groups, seen);
//         }
//         else {
//             return solve2(row, counts, next_block, count_index, remaining);//, groups, seen);
//         }
//     }
// }
type Cache = HashMap<(String, String), usize>;
fn solve2(row: &[char], counts: &[usize], remaining: i64, seen: &mut Cache) -> usize {
    // println!("{:?}, {:?}", row, counts);
    let key = cache(row, counts);
    if seen.contains_key(&key) {
        return seen.get(&key).unwrap().clone();
    }
    let mut remaining = remaining;
    if counts.iter().sum::<usize>() as i64 > remaining {
        // println!("counts more than remaining: {:?} vs {remaining}", counts.iter().sum::<usize>());
        return 0;
    }
    if row.is_empty() {
        if counts.is_empty() {
            return 1;
        }
        return 0;
    }
    if counts.is_empty() {
        if row.contains(&'#') {
            return 0;
        }
        return 1;
    }
    let count = counts[0];
    if count > row.len() {
        return 0;
    }
    let mut ret = 0;
    // println!("{:?} contains {}? {}", row, count, contains_count(row, count));
    if contains_count(row, count) {
        if row.len() == count || row[count] != '#' {
            if row.len() == count {
                let ways = solve2(&row[count..], &counts[1..], remaining - count as i64, seen);
                let key = cache(&row[count..], &counts[1..]);
                seen.insert(key, ways);
                ret += ways;
            }
            else {
                let ways = solve2(&row[count+1..], &counts[1..], remaining - count as i64, seen);
                let key = cache(&row[count+1..], &counts[1..]);
                seen.insert(key, ways);
                ret += ways;
            }
        }
    }

    if row[0] == '#' {
        return ret;
    }
    if row[0] == '?' {
        remaining -= 1;
    }
    let ways = solve2(&row[1..], counts, remaining, seen);
    let key = cache(&row[1..], counts);
    seen.insert(key, ways);
    return ret + ways;
}

fn contains_count(row: &[char], count: usize) -> bool {
    for i in 0..count {
        if i >= row.len() {
            return false;
        }
        if row[i] == '.' {
            return false;
        }
    }
    return true;
}

fn cache (row: &[char], counts: &[usize]) -> (String, String) {
    return (row.iter().collect::<String>(), counts.iter().join(","));
}

// fn get_block_size(row: &mut Vec<char>, row_index: usize) -> (usize, usize) {
//     let mut size = 0;
//     let mut hash_count = 0;
//     let mut row_index = row_index;
//     while row_index < row.len() {
//         if row[row_index] == '.' {
//             break;
//         }
//         if row[row_index] == '#' {
//             hash_count += 1;
//         }
//         size += 1;
//         row_index += 1;
//     }
//     return (size, hash_count);
// }

// fn get_next_block(row: &Vec<char>, row_index: usize) -> Result<usize> {

//     let mut row_index = row_index;
//     while row_index < row.len() {
//         if row[row_index] != '.' {
//             return Ok(row_index);
//         }
//         row_index += 1;
//     }
//     Err(anyhow!("no more blocks"))
// }
// 742420 too low