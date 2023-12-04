use std::str;
use std::collections::HashMap;
use std::collections::HashSet;

fn get_numbers(words: Vec<&str>) -> Vec<u32> {
    let mut numbers: Vec<u32> = Vec::new();
    for word in words {
        match word.parse::<u32>() {
            Ok(v) => {
                numbers.push(v);
            },
            Err(_) => ()
        };
    }
    return numbers;
}

pub fn advent(data: String) -> u32 {
    let mut result = 0;
    for line in data.lines() {
        let cards: Vec<&str> = line.split('|').collect();
        let card_a: Vec<&str> = cards[0].split(' ').collect();
        let card_b: Vec<&str> = cards[1].split(' ').collect();
        let card_a = get_numbers(card_a);
        let card_b = get_numbers(card_b);
        let winning_numbers: HashSet<u32> = HashSet::from_iter(card_a.iter().cloned());

        let mut score = 0;
        for number in card_b {
            if winning_numbers.contains(&number) {
                if score == 0 {
                    score = 1;
                }
                else {
                    score *= 2;
                }
            }
        }

        result += score;

    }
    return result;
}

pub fn advent_2(data: String) -> u32 {
    let mut dict: HashMap<u32, u32> = HashMap::new();
    let lines: Vec<&str> = data.split('\n').collect();
    let mut result = lines.len() as u32;
    for (i, line) in lines.iter().enumerate().rev() {
        let cards: Vec<&str> = line.split('|').collect();
        let card_a: Vec<&str> = cards[0].split(' ').collect();
        let card_b: Vec<&str> = cards[1].split(' ').collect();
        let card_a = get_numbers(card_a);
        let card_b = get_numbers(card_b);
        let winning_numbers: HashSet<u32> = HashSet::from_iter(card_a.iter().cloned());

        let mut score: u32 = 0;
        for number in card_b {
            if winning_numbers.contains(&number) {
                score += 1;
            }
        }
        let count = score.clone() as usize;
        for n in i + 1.. i + count + 1 {
            let n = n as u32;
            let cur = dict.get(&n).unwrap_or(&0);
            score += cur;
        }
        dict.insert(i as u32, score);
        result += score;
    }
    return result;
}