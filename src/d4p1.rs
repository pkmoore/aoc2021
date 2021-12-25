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
    let nums = data
        .trim()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    // Remove first separating new line
    reader.read_line(&mut data)?;

    let mut data = String::new();
    reader.read_to_string(&mut data)?;
    let mut boards = data
        .split_whitespace()
        .map(|x| (x.parse::<u32>().unwrap(), false))
        .collect::<Vec<(u32, bool)>>();

    let mut result:Option<&[(u32, bool)]> = None;
    let mut latest_num = 0;
    'outer: for i in nums {
        latest_num = i;
        for j in 0..boards.len() {
            if boards[j].0 == i {
                boards[j] = (boards[j].0, true)
            }
        }
        result = check_boards(&boards);
        if result.is_some() {
            latest_num = i;
            break 'outer;
        }
    }

    println!("D4P1 Result: {}", score_board(result.unwrap(), latest_num));

    Ok(())
}

fn score_board(b: &[(u32, bool)], latest_num: u32) -> u32 {
    let mut acc: u32 = 0;
    for i in b {
        if i.1 == false {
            acc += i.0;
        }
    }
    acc * latest_num
}

fn check_boards(b: &[(u32, bool)]) -> Option<&[(u32, bool)]> {
    for i in (0..b.len()).step_by(25) {
        let cur_board = &b[i..i+25];
        if check_board(cur_board) {
            return Some(cur_board);
        }
    }
    None
}

fn check_board(b: &[(u32, bool)]) -> bool {
    // Horizontal Wins
    for i in (0..b.len()).step_by(5) {
        if b[i..i+5].iter().all(|&x| x.1 == true) {
            return true;
        }
    }

    // Vertical wins
    let mut winner = true;
    // For each column... 
    for i in 0..5 {
        // ...assume we have a winner
        winner = true;
        // Look through each cell in the column
        for j in 0..5 {
            // This indexing expression jumps through the 1D array to hit all the cells in the column
            if b[(j * 5) + i].1 == false {
                // If we get any that are unmarked (aka b[...].1 == false) we note that the column isn't a winner and bail out of this inner loop
                winner = false;
                break;
            }
        }
        // If we get to here and winner is true, then all of the column's cells are marked so we have a winner
        if winner {
            break;
        }
    }
    // At this point, we DON'T have a horizontal winner so we can only have
    // a vertical winner. This means that the result of the above loop 
    // (which finds vertical winners) is all we need to return
    winner
}

#[allow(dead_code)]
fn print_boards(b: Vec<(u32, bool)>) {
    for i in (0..b.len()).step_by(25) {
        print_board(&b[i..i+25]);
        println!();
    }
}

#[allow(dead_code)]
fn print_board(b: &[(u32, bool)]) {
    assert!(b.len() == 25, "Cannot print illegal board size");
    for i in 0..5 {
        for j in 0..5 {
            let t = b[(i * 5) + j];
            print!(" {} {}", t.0, t.1);
        }
        println!();
    }
}
