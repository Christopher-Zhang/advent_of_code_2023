use std::cmp::Ordering;
use std::str;

pub async fn advent(data: String) -> u64 {
    let mut answer = 0;
    let mut order: Vec<(u64, &str, u64)> = Vec::new();
    for line in data.lines() {
        let groups: Vec<&str> = line.split(' ').collect();
        let hand = groups[0];
        let rank = parse_hand(hand);
        let bet = groups[1].parse::<u64>().unwrap();
        order.push((rank, hand, bet));
    }

    order.sort_by(|a, b| {
        let mut ordering = a.0.cmp(&b.0);
        let mut index = 0;
        while ordering == Ordering::Equal && index < 6 {
            let c = get_priority(a.1.chars().nth(index).unwrap());
            let d = get_priority(b.1.chars().nth(index).unwrap());
            ordering = c.cmp(&d);
            index += 1;
        }
        return ordering;
    });
    for (i, hand) in order.iter().enumerate() {
        answer += hand.2 * (i as u64 + 1);
    }
    return answer;
}

pub async fn advent_2(data: String) -> u64 {
    let mut answer = 0;
    let mut order: Vec<(u64, &str, u64)> = Vec::new();
    for line in data.lines() {
        let groups: Vec<&str> = line.split(' ').collect();
        let hand = groups[0];
        let rank = parse_hand_2(hand);
        let bet = groups[1].parse::<u64>().unwrap();
        order.push((rank, hand, bet));
    }

    order.sort_by(|a, b| {
        let mut ordering = a.0.cmp(&b.0);
        let mut index = 0;
        while ordering == Ordering::Equal && index < 5 {
            let c = get_priority_2(a.1.chars().nth(index).unwrap());
            let d = get_priority_2(b.1.chars().nth(index).unwrap());
            ordering = c.cmp(&d);
            index += 1;
        }
        return ordering;
    });

    for i in 0 .. order.len() {
        let x = order.get(i).unwrap();
        if x.1.chars().collect::<Vec<char>>().contains(&'J') && x.0 == 6 {

            dbg!(order.get(i));
        }
    }
    for (i, hand) in order.iter().enumerate() {
        answer += hand.2 * (i as u64 + 1);
    }
    return answer;
}

fn parse_hand(hand: &str) -> u64 {
    let mut counts = [0; 13];
    for c in hand.chars() {
        let index = get_priority(c);
        counts[index as usize] += 1;
    }
    let mut pair_count = 0;
    let mut set_count = 0;
    for count in counts {
        if count == 5 {
            return 6;
        }
        if count == 4 {
            return 5;
        }
        if count == 3 {
            set_count += 1;
        }
        if count == 2 {
            pair_count += 1;
        }   
    }
    if set_count == 1 {
        if pair_count == 1 {
            return 4;
        }
        return 3;
    }
    
    if pair_count == 2 {
        return 2;
    }

    if pair_count == 1 {
        return 1;
    }
    return 0;


}
fn parse_hand_2(hand: &str) -> u64 {
    let mut counts: [u64; 13] = [0; 13];
    let mut j_count = 0;
    for c in hand.chars() {
        if c == 'J' {
            j_count += 1;
        }
        else 
        {
            let index = get_priority_2(c);
            counts[index as usize] += 1;
        }
    }

    counts.sort_by(|a,b| b.cmp(&a));
    let first = counts[0];
    let second = counts[1];
    if first + j_count > 3 {
        return first + j_count + 1;
    }
    if first + j_count == 3 {
        if second == 2 {
            return 4
        }
        return 3;
    }
    if first + j_count == 2 {
        if second == 2 {
            return 2;
        }
        return 1;
    }
    return 0;


}

fn get_priority(c: char) -> u64 {
    let priority = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
    priority.iter().position(|&r| r == c).unwrap_or(0) as u64
}

fn get_priority_2(c: char) -> u64 {
    // return get_priority(c);
    let priority = ['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];
    priority.iter().position(|&r| r == c).unwrap_or(0) as u64
}

