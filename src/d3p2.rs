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
    let most_common_bits = col_vecs.iter()
        .map(|x| {
            let s: u32 = x.iter().map(|c| c.to_digit(10).unwrap()).sum();
            match s > col_length as u32 / 2 {
                true => '1',
                false => '0',
            }
        })
        .collect::<Vec<char>>();
    
    let mut tmp_line_vec = data.clone();
    let mut cur_index = 0;
    while tmp_line_vec.len() > 1 && cur_index < most_common_bits.len() {
        println!("{}", tmp_line_vec.len());
        tmp_line_vec = tmp_line_vec.iter().filter(|x| x[cur_index] == most_common_bits[cur_index]).cloned().collect();
        cur_index += 1;
    }
    println!("{}", tmp_line_vec.len());
    println!("{:#?}", tmp_line_vec);

    Ok(())
}

fn column_vec_from_lines(index: usize, lines: &Vec<Vec<char>>) -> Vec<char> {
    lines.iter().map(|x| x[index]).collect::<Vec<char>>()
}
