use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("./data/day1.txt")?;
    let reader = BufReader::new(file);
    let data = reader
        .lines()
        .map(|x| x.unwrap().trim().parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let increases = data.windows(2).filter(|x| x[1] > x[0]).count();
    println!("Day 1 Part 1 Answer: {}", increases);
    Ok(())
}
