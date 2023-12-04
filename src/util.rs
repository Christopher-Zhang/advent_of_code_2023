use anyhow::Result;
use reqwest::header::USER_AGENT;
use reqwest::{cookie::Jar, Url};
use std::fs;

pub fn path_exists(path: &str) -> bool {
    let metadata = fs::metadata(path);
    match metadata {
        Ok(v) => v.is_file(),
        Err(_) => false
    }
}

pub async fn get_problem_data(day_number: u32) -> Result<String> {
    let session_key = String::from("session=53616c7465645f5f4f664a3cf461c6f3cb105236372ce07bdbf5e275f2ffffa1dff02c9afeeb41f5a08de22f8b5ea9cbef1c22451964f0bb6f565ef85df63d0b");
    let day = day_number.to_string();
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
        
        fs::write(path, body).expect("Unable to write");
        data = fs::read_to_string(path).expect("unable to read file");
    }

    Ok(data)
}