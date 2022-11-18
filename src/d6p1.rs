use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("./data/day6.txt")?;
    let mut reader = BufReader::new(file);
    let mut data = String::new();
    reader.read_to_string(&mut data)?;
    let mut fish = data.split(",").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    // Straightfoward approach, fish vec grows according to rules for 80 days
    // loop through teach element every tick and update accordingly
    for _day in 0..80 {
        tick(&mut fish);
    }

    println!("D6P1 Result: {}", fish.len());
    
    Ok(())
}

fn tick(fish: &mut Vec<u32>) {
    let mut new_fish_count: usize = 0;
    for f in fish.iter_mut() {
        if *f == 0 {
            *f = 6;
            new_fish_count += 1;
        } else {
            *f -= 1;
        }
    }
    fish.extend_from_slice(&vec![8; new_fish_count]);
}