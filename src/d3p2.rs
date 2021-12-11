use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use std::cmp::Ordering;

extern crate num_integer;
use crate::num_integer::Integer;

fn main() -> std::io::Result<()> {
    let file = File::open("./data/day3.txt")?;
    let reader = BufReader::new(file);
    let data = reader
        .lines()
        .map(|x| x.unwrap().trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let line_length = data[0].len();
    let col_vecs = (0..(line_length))
        .map(|x| column_vec_from_lines(x, &data))
        .collect::<Vec<Vec<char>>>();

    let (oxy, co2) = equipment_ratings(&data, &col_vecs);

    println!("D3P2 Result: {}", oxy * co2);

    Ok(())
}

fn column_vec_from_lines(index: usize, lines: &[Vec<char>]) -> Vec<char> {
    lines.iter().map(|x| x[index]).collect::<Vec<char>>()
}

fn most_bit(col: &[char]) -> char {
    let line_length = col.len() as u32;
    let s: u32 = col.iter().map(|c| c.to_digit(10).unwrap()).sum();
    let midpoint = line_length.div_ceil(&2);
    match s.cmp(&midpoint) {
        Ordering::Less => '0',
        Ordering::Greater => '1',
        Ordering::Equal => '1',
    }
}

fn least_bit(col: &[char]) -> char {
    let line_length = col.len() as u32;
    let s: u32 = col.iter().map(|c| c.to_digit(2).unwrap()).sum();
    let midpoint = line_length.div_ceil(&2);
    match s.cmp(&midpoint) {
        Ordering::Less => '1',
        Ordering::Greater => '0',
        Ordering::Equal => '0',
    }
}

fn equipment_ratings(lines: &[Vec<char>], cols: &[Vec<char>]) -> (i64, i64) {
    let mut oxy_lines = lines.to_owned();
    let mut co2_lines = lines.to_owned();
    let mut col_index = 0;

    while (oxy_lines.len() > 1 || co2_lines.len() > 1) && col_index < cols.len() {
        if oxy_lines.len() > 1 {
            let oxy_col = column_vec_from_lines(col_index, &oxy_lines);
            let oxy_bit = most_bit(&oxy_col);
            oxy_lines = oxy_lines
                .iter()
                .filter(|x| x[col_index] == oxy_bit)
                .cloned()
                .collect::<Vec<Vec<char>>>();
        }
        if co2_lines.len() > 1 {
            let co2_col = column_vec_from_lines(col_index, &co2_lines);
            let co2_bit = least_bit(&co2_col);
            co2_lines = co2_lines
                .iter()
                .filter(|x| x[col_index] == co2_bit)
                .cloned()
                .collect::<Vec<Vec<char>>>();
        }
        col_index += 1;
    }
    assert!(oxy_lines.len() == 1);
    assert!(co2_lines.len() == 1);
    (
        isize::from_str_radix(&oxy_lines[0].iter().collect::<String>(), 2).unwrap() as i64,
        isize::from_str_radix(&co2_lines[0].iter().collect::<String>(), 2).unwrap() as i64,
    )
}
