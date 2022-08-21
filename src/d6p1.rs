use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("./data/day6.txt")?;
    let mut reader = BufReader::new(file);
    let mut data = String::new();
    reader.read_to_string(&mut data)?;
    let mut fish = data.split(",").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    println!("D6P1 Result: {}", 0);
    
    Ok(())
}