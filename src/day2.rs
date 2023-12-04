use std::str;
use std::fs;
use regex::Regex;

pub fn path_exists(path: &str) -> bool {
    let metadata = fs::metadata(path);
    match metadata {
        Ok(v) => v.is_file(),
        Err(_) => false
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "./input";
    let data;
    data = fs::read_to_string(path).unwrap();

    let mut sum = 0;
    for line in data.lines() {
        let parts: Vec<&str> = line.split(":").collect();
        let number = Regex::new(r"\d+").unwrap();
        let caps = number.captures(parts[0]).unwrap();

        // let game_id = &caps[0];

        let turns: Vec<&str> = parts[1].split(";").collect();
        let green_regex = Regex::new(r"(\d+) green").unwrap();
        let red_regex = Regex::new(r"(\d+) red").unwrap();
        let blue_regex = Regex::new(r"(\d+) blue").unwrap();
        let mut max_blue = 0;
        let mut max_green = 0;
        let mut max_red = 0;

        for turn in turns {
            let blue = match blue_regex.captures(turn) {
                Some(v) => v[1].parse::<i32>().unwrap_or(0),
                None => 0
            };
            let red = match red_regex.captures(turn) {
                Some(v) => v[1].parse::<i32>().unwrap_or(0),
                None => 0
            };
            let green = match green_regex.captures(turn) {
                Some(v) => v[1].parse::<i32>().unwrap_or(0),
                None => 0
            };
            
            if blue > max_blue {
                max_blue = blue.clone();
            }
            if green > max_green {
                max_green = green.clone();
            }
            if red > max_red {
                max_red = red.clone();
            }
        }

        sum += max_blue * max_green * max_red;
    }
    
    println!("Final total: {}", sum);
    // let mut red = 12;
    // let mut green = 13;
    // let mut blue = 14;
    Ok(())
}
