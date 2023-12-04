use std::collections::HashMap;
use std::str;
use std::fs;
use reqwest;
use reqwest::header::USER_AGENT;
use reqwest::{cookie::Jar, Url};

pub fn path_exists(path: &str) -> bool {
    let metadata = fs::metadata(path);
    match metadata {
        Ok(v) => v.is_file(),
        Err(_) => false
    }
    // metadata.unwrap().is_file()
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session_key = String::from("session=53616c7465645f5f4f664a3cf461c6f3cb105236372ce07bdbf5e275f2ffffa1dff02c9afeeb41f5a08de22f8b5ea9cbef1c22451964f0bb6f565ef85df63d0b");
    let day = String::from("1");
    let mut uri = String::from("https://adventofcode.com/2023/day/");
    uri.push_str(&day);
    uri.push_str("/input");
    let path = "./input";
    let data;
    if path_exists(path) {
        data = fs::read_to_string(path).expect("unable to read file");
    }
    else {
        let jar = Jar::default();
        let url = &uri.parse::<Url>().unwrap();
        jar.add_cookie_str(&session_key, url);
        let client = reqwest::Client::new();
        let body = client
            .get(uri)
            .header(USER_AGENT, "chrisconcord@gmail.com")
            .send()
            .await?
            .text()
            .await?;
        // let body = reqwest::get(uri)
        //     .await?
        //     .text()
        //     .await?;
        
        fs::write(path, body).expect("Unable to write");
        data = fs::read_to_string(path).expect("unable to read file");
    }
    let dict = HashMap::from([("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9),  ("1", 1), ("2", 2), ("3", 3), ("4", 4), ("5", 5), ("6", 6), ("7", 7), ("8", 8), ("9", 9)]);

    // print!("{:?}", data);
    let mut sum = 0;
    // for line in data.lines() {
    //     let mut first = 0;
    //     let mut last = 0;
    //     let mut seen_first = false;
    //     println!("line: {:?}", line);
    //     for c in line.chars() {
    //         if c.is_digit(10) {
    //             if !seen_first {
    //                 first = c.to_digit(10).unwrap();
    //                 seen_first = true;
    //             }
    //             last = c.to_digit(10).unwrap();
    //         }
    //     }
    //     print!("first {:?} last {:?}\n", first, last);
    //     let num = first * 10 + last;
    //     sum += num;
    // }

    for line in data.lines() {
        let mut first = 0;
        let mut last = 0;
        let mut first_index = std::i32::MAX;
        let mut last_index: i32 = -1;
        for (word, number) in dict.clone().into_iter() {
            let mut find = 0;
            let res = line.find(word);
            match res {
                Some(v) => find = i32::try_from(v).unwrap(),
                None => continue,
            }
            if find < first_index {
                first_index = find;
                first = number;
            }
            let res = line.rfind(word);
            match res {
                Some(v) => find = i32::try_from(v).unwrap(),
                None => continue,
            }
            if find > last_index {
                last_index = find;
                last = number;
            }
        }
        print!("first {:?} last {:?}\n", first, last);
        let num = first * 10 + last;
        sum += num;
    }
    print!("answer: {:?}", sum);
    Ok(())
}
