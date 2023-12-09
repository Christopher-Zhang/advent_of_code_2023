use std::str;
use std::collections::{VecDeque, HashMap};
use regex::Regex;
use std::time::Instant;
pub async fn advent(data: String) -> u32 {
    let mut lines: VecDeque<&str> = data.lines().collect();
    let instruction = lines.pop_front().unwrap();
    lines.pop_front();
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let re = Regex::new(r"(\w\w\w) = \((\w\w\w), (\w\w\w)\)").unwrap();
    for line in lines{
        let caps = re.captures(line).unwrap();
        map.insert(caps[1].to_string(), (caps[2].to_string(), caps[3].to_string()));
    }
    let mut steps = 0;
    let mut current = "AAA";
    let mut i = 0;
    while current != "ZZZ" {
        steps += 1;
        let command = instruction.chars().nth(i).unwrap();
        i += 1;
        i %= instruction.len();

        if command == 'R' {
            current = &map.get(current).unwrap().1;
        }
        if command == 'L' {
            current = &map.get(current).unwrap().0;
        }
    }
    return steps;
}

pub async fn advent_2(data: String) -> u64 {
    let mut steps: u64 = 0;
    let mut lines: VecDeque<&str> = data.lines().collect();
    let instruction = lines.pop_front().unwrap();
    lines.pop_front();
    let mut map: HashMap<(char, char, char), ((char,char,char), (char,char,char))> = HashMap::new();
    let re = Regex::new(r"(\w\w\w) = \((\w\w\w), (\w\w\w)\)").unwrap();
    let mut currents: Vec<(char, char, char)> = Vec::new();
    for line in lines{
        let caps = re.captures(line).unwrap();
        map.insert(get_chars(&caps[1]), (get_chars(&caps[2]), get_chars(&caps[3])));
        if caps[1].ends_with('A') {
            currents.push(get_chars(&caps[1]));
        }
    }
    let starts = currents.clone();
    let endpoints: Vec<Vec<u64>> = Vec::new();
    for _ in starts {
        endpoints.push(Vec::<u64>::new());
    }
    let mut i = 0;
    let now = Instant::now();
    while !all_z(&currents) {
        steps += 1;
        let command = instruction.chars().nth(i).unwrap();
        i += 1;
        i %= instruction.len();
        for x in 0..currents.len() {
            if command == 'R' {
                currents[x] = map.get(&currents[x]).unwrap().1;
            }
            if command == 'L' {
                currents[x] = map.get(&currents[x]).unwrap().0;
            }
        }
        if currents[x].2 == 'Z' {
            endpoints[x].push(steps);
        }
        // check the periods of each start point and then get the LCM for all
    }
    return steps;
}

fn all_z(currents: &Vec<(char,char,char)>) -> bool {
    for node in currents {
        if node.2 != 'Z' {
            return false;
        }
    }
    return true;
}
fn get_chars(st: &str) -> (char, char, char) {
    let chars: Vec<char> = st.chars().collect();
    return (chars[0], chars[1], chars[2]);
}