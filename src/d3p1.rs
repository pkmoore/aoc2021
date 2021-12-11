use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("./data/day3.txt")?;
    let reader = BufReader::new(file);
    let data = reader
        .lines()
        .map(|x| x.unwrap().trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let line_length = data[0].len();
    let col_length = data.len();
    let col_vecs = (0..(line_length))
        .map(|x| column_vec_from_lines(x, &data))
        .collect::<Vec<Vec<char>>>();
    let epsilon = col_vecs
        .iter()
        .map(|x| {
            let s: u32 = x.iter().map(|c| c.to_digit(10).unwrap()).sum();
            match s > col_length as u32 / 2 {
                true => '1',
                false => '0',
            }
        })
        .collect::<String>();
    let gamma = epsilon
        .chars()
        .map(|x| match x {
            '0' => '1',
            '1' => '0',
            _ => 'X',
        })
        .collect::<String>();
    let epsilon = isize::from_str_radix(&epsilon, 2).unwrap();
    let gamma = isize::from_str_radix(&gamma, 2).unwrap();
    println!("Power Rating: {}", epsilon * gamma);

    Ok(())
}

fn column_vec_from_lines(index: usize, lines: &Vec<Vec<char>>) -> Vec<char> {
    lines.iter().map(|x| x[index]).collect::<Vec<char>>()
}
