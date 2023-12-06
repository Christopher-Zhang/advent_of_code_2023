use std::str;
use std::collections::VecDeque;

pub async fn advent(data: String) -> u32 {
    let rows: Vec<&str> = data.split('\n').collect();
    let mut times: VecDeque<&str> = rows[0].split_whitespace().collect();
    let mut distances: VecDeque<&str> = rows[1].split_whitespace().collect();
    let mut product = 1;
    times.pop_front();
    distances.pop_front();
    for (time, distance) in times.iter().zip(distances.iter()) {
        println!("  time: {} distance: {}", time, distance);
        let time = time.parse::<u32>().unwrap();
        let distance = distance.parse::<u32>().unwrap();
        let mut start = 0;
        let mut end = time.clone();

        for i in 1..time {
            let this_distance = get_distance(i, time);
            if this_distance > distance {
                start = i;
                break;
            }
        }
        for i in (start+1..time).rev() {
            let this_distance = get_distance(i, time);
            if this_distance > distance {
                end = i;
                break;
            }
        }
        let ways = end - start + 1;
        product *= ways;
    }
    return product;
}

pub async fn advent_2(data: String) -> u64 {
    let rows: Vec<&str> = data.split('\n').collect();

    let mut times: VecDeque<&str> = rows[0].split_whitespace().collect();
    let mut distances: VecDeque<&str> = rows[1].split_whitespace().collect();
    let mut product = 1;
    times.pop_front();
    distances.pop_front();

    let mut time = String::from("");
    for t in times {
        time.push_str(t);
    }
    let mut distance = String::from("");
    for d in distances {
        distance.push_str(d);
    }
    remove_whitespace(&mut distance);
    remove_whitespace(&mut time);
    println!("  time: {} distance: {}", time, distance);
    let time = time.parse::<u64>().unwrap();
    let distance = distance.parse::<u64>().unwrap();

    let mut start = 0;
    let mut end = time.clone();

    for i in 1..time {
        let this_distance = get_distance_u64(i, time);
        if this_distance > distance {
            start = i;
            break;
        }
    }

    for i in (start+1..time).rev() {
        let this_distance = get_distance_u64(i, time);
        if this_distance > distance {
            end = i;
            break;
        }
    }
    let ways = end - start + 1;
    product *= ways;
    return product;
}

fn get_distance_u64(hold_time: u64, time: u64) -> u64 {
    if hold_time >= time {
        return 0;
    }
    return hold_time * (time - hold_time);
}
fn get_distance(hold_time: u32, time: u32) -> u32 {
    if hold_time >= time {
        return 0;
    }
    return hold_time * (time - hold_time);
}

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}