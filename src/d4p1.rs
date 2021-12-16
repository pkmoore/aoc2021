use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

extern crate itertools;

fn main() -> std::io::Result<()> {
    let file = File::open("./data/day4.txt")?;
    let mut reader = BufReader::new(file);
    let mut data: String = String::new();
    reader.read_line(&mut data)?;
 
    // Remove line of numbers
    let nums = data.trim().split(',').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    // Remove first separating new line
    reader.read_line(&mut data)?;

    let mut data = String::new();
    reader.read_to_string(&mut data)?;
    let boards = data.split_whitespace().map(|x| (x.parse::<u32>().unwrap(), false)).collect::<Vec<(u32, bool)>>();


    for i in (0..boards.len()).step_by(25) {
        print_board(&boards[i..i+25]);
        println!();

   
    }




    println!("D4P1 Result: {}", 0);

    Ok(())
}

fn print_board(b: &[(u32, bool)])  {
    assert!(b.len() == 25, "Cannot print illegal board size");
    for i in 0..5 {
        for j in 0..5 {
            print!("{} ", b[(i * 5) + j].0);
        }
        println!();
    }

}