use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("./data/day6.txt")?;
    let mut reader = BufReader::new(file);
    let mut data = String::new();
    reader.read_to_string(&mut data)?;
    let fish = data.split(",").map(|x| x.parse::<u8>().unwrap()).collect::<Vec<u8>>();

    let mut fish_at_each_day = vec![0u128; 9];

    for f in fish {
        fish_at_each_day[f as usize] += 1;
    }

    // Because we are now keeping track of the number of fish for each day,
    // a tick is simply increasing the number of fish in day 8 by the number of fish in day 0
    // then rotating left
    for _day in 0..256 {
        tick(&mut fish_at_each_day);
    }

    let total_fish: u128 = fish_at_each_day.iter().fold(0u128, | sum, x | sum +  *x as u128);

    println!("D6P1 Result: {}", total_fish);
    
    Ok(())
}

fn tick(fish_at_each_day: &mut Vec<u128>) {
    fish_at_each_day[7] += fish_at_each_day[0];
    fish_at_each_day.rotate_left(1);
}