use std::time::Instant;
use std::fs;
use anyhow::Result;

use crate::day17::{advent, advent_2};
mod day17;
// mod util;

#[tokio::main]
async fn main() -> Result<()> {
    let path = "./input";
    let data = fs::read_to_string(path).unwrap();

    let now = Instant::now();
    let result = advent(data.clone()).await;
    println!("Final total part 1: {}", result);
    let elapsed = now.elapsed();
    println!("Part 1 finished in {:.2?}", elapsed);
 
    let now = Instant::now();
    let result = advent_2(data).await;
    println!("Final total part 2: {}", result);
    let elapsed = now.elapsed();
    println!("Part 2 finished in {:.2?}", elapsed);

    Ok(())
}
