use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use std::cmp::Ordering;


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

    let oxy = equipment_rating(&data, &oxygen_selected_bits(&col_vecs));
    let co2 = equipment_rating(&data, &co2_selected_bits(&col_vecs));

    println!("D3P2 Result: {}", oxy * co2);


    Ok(())
}

fn column_vec_from_lines(index: usize, lines: &Vec<Vec<char>>) -> Vec<char> {
    lines.iter().map(|x| x[index]).collect::<Vec<char>>()
}

fn oxygen_selected_bits(lines: &Vec<Vec<char>>) -> Vec<char> {
    let line_length = lines[0].len() as u32;
    let out = lines.iter()
        .map(|x| {
            let s: u32 = x.iter()
                          .map(|c| c.to_digit(10)
                                    .unwrap())
                          .sum();
            match s.cmp(&(line_length / 2)) {
                Ordering::Less => '0',
                Ordering::Greater => '1',
                Ordering::Equal => '1', //break ties with '1' for oxy
            }
        })
    .collect();
    println!("oxy bits {:#?}", out);
    out
}

fn co2_selected_bits(lines: &Vec<Vec<char>>) -> Vec<char> {
    let line_length = lines[0].len() as u32;
    let out = lines.iter()
        .map(|x| {
            let s: u32 = x.iter()
                          .map(|c| c.to_digit(10)
                                    .unwrap())
                          .sum();
            // We flip the result of less and greater here because
            // the co2 scrubber wants the LEAST common bits and our
            // math trick finds the most common
            match s.cmp(&(line_length / 2)) {
                Ordering::Less => '1',  //'1' rather than '0'
                Ordering::Greater => '0', // '0' rather than '1'
                Ordering::Equal => '0', // break ties with '0' for co2
            }
        })
    .collect();
    println!("CO2 bits {:#?}", out);
    out
}

fn equipment_rating(lines: &Vec<Vec<char>>, filter_bits: &Vec<char>, ) -> i64 {
    let mut tmp_line_vec = lines.clone();
    let mut cur_index = 0;
    while tmp_line_vec.len() > 1 && cur_index < filter_bits.len() {
        tmp_line_vec = tmp_line_vec.iter()
                                   .filter(|x| x[cur_index] == filter_bits[cur_index])
                                   .cloned()
                                   .collect();
        cur_index += 1;
    }
    let final_bit_string = tmp_line_vec[0].iter().collect::<String>();
    isize::from_str_radix(&final_bit_string, 2).unwrap() as i64
}
