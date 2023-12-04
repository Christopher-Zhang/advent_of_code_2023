use std::str;
use std::fs;
use anyhow::Result;

use crate::day4::{advent, advent_2};
// use crate::util::get_problem_data;
mod day4;
mod util;

#[tokio::main]
async fn main() -> Result<()> {
    let path = "./input";
    let data = fs::read_to_string(path).unwrap();

    let result = advent(data.clone());
    println!("Final total part 1: {}", result);

    let result = advent_2(data);
    println!("Final total part 2: {}", result);
    Ok(())
}
