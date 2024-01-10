use std::time::Instant;
use std::fs;
use anyhow::Result;
#[macro_use]
extern crate lazy_static;
use crate::day21::{advent, advent_2};
mod day21;

#[tokio::main]
async fn main() -> Result<()> {
    let test = "./test";
    let path = "./input";
    let test_data = fs::read_to_string(test).unwrap();
    let input_data = fs::read_to_string(path).unwrap();
    dbg!(-1 % 10);
    println!("Test results: ");
    {
        let now = Instant::now();
        let result = advent(test_data.clone()).await;
        println!("Final total part 1: {}", result);
        let elapsed = now.elapsed();
        println!("Part 1 finished in {:.2?}", elapsed);
     
        let now = Instant::now();
        let result = advent_2(test_data).await;
        println!("Final total part 2: {}", result);
        let elapsed = now.elapsed();
        println!("Part 2 finished in {:.2?}", elapsed);
    }
    if input_data.len() > 0 {
        println!();
        println!();
        println!("Input results: ");
        let now = Instant::now();
        let result = advent(input_data.clone()).await;
        println!("Final total part 1: {}", result);
        let elapsed = now.elapsed();
        println!("Part 1 finished in {:.2?}", elapsed);
     
        let now = Instant::now();
        let result = advent_2(input_data).await;
        println!("Final total part 2: {}", result);
        let elapsed = now.elapsed();
        println!("Part 2 finished in {:.2?}", elapsed);
    }

    Ok(())
}
