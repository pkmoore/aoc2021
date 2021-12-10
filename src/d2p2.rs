use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("./data/day2.txt")?;
    let reader = BufReader::new(file);
    let data = reader
        .lines()
        .map(|x| {
            x.unwrap()
                .split(" ")
                .map(|y| y.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    let mut x_coord = 0;
    let mut y_coord = 0;
    let mut aim = 0;
    for row in data {
        let direction = &row[0];
        let amount = row[1].parse::<u64>().unwrap();
        match direction.as_str() {
            "forward" => {
                x_coord += amount;
                y_coord += aim * amount
            }
            "up" => aim -= amount,
            "down" => aim += amount,
            _ => panic!("Bad direction"),
        }
        println!("X: {} Y: {} Aim: {}", x_coord, y_coord, aim)
    }
    println!("Result: {}", x_coord * y_coord);
    Ok(())
}
