use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

extern crate itertools;

fn main() -> std::io::Result<()> {
    let file = File::open("./data/day5.txt")?;
    let mut reader = BufReader::new(file);
    let mut data = String::new();
    reader.read_to_string(&mut data)?;
    let lines = data.lines()
                    // We have lines, split them on " -> " including the spaces
                    .map(|x| x.split(" -> ")
                              // This gives us ["xxx,yyy", "xxx,yyy"], for each xxx,yyy combo split on ,
                              .map(|unit| unit.split(",").map(|num| num.parse::<u32>().unwrap()).collect::<Vec<u32>>())
                                  .collect::<Vec<Vec<u32>>>())
                        .collect::<Vec<Vec<Vec<u32>>>>();

    println!("{:?}\n", lines);
    Ok(())
}