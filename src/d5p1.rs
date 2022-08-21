use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
use std::cmp::max;

extern crate num;
use crate::num::Integer;

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
                              .map(|unit| unit.split(",").map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>())
                                  .collect::<Vec<Vec<i32>>>())
                        .collect::<Vec<Vec<Vec<i32>>>>();

    
    let mut coords: HashMap<Vec<i32>, u32> = HashMap::new();


    for line in lines { 
        // For Part 1, we are only concerned with horizontal or vetical lines
        // i.e. lines where x1 == x2 or y1 == y2
        if line[0][0] == line[1][0] || line[0][1] == line[1][1] {
            let points = get_segment_points(&line[0], &line[1]);
            for point in points {
                if let Some(x) = coords.get_mut(&point) {
                    *x += 1;
                } else {
                    coords.insert(point, 1);
                }
            }
        }
    }
   println!("D5P1 Result: {}", coords.iter().filter(|x| x.1 >= &2).count());
    
    Ok(())
}

fn get_slope(p1: &Vec<i32>, p2: &Vec<i32>) -> Vec<i32> {
    let y_part = p2[1] - p1[1];
    let x_part = p2[0] - p1[0];
    // if GCD is 0, use 1 instead so the division on the next line leaves x and y unaltered
    let gcd = max(y_part.gcd(&x_part), 1);
    return vec![x_part / gcd, y_part / gcd];
}

fn get_segment_points(p1: &Vec<i32>, p2: &Vec<i32>) -> Vec<Vec<i32>> {
    let slope = get_slope(p1, p2);
    let mut points: Vec<Vec<i32>> = Vec::new();
    let mut cur_x = p1[0];
    let mut cur_y = p1[1];
    points.push(vec![cur_x, cur_y]);
    while cur_x != p2[0] || cur_y != p2[1] {
        cur_x += slope[0];
        cur_y += slope[1];
        points.push(vec![cur_x, cur_y]);
    }
    points
}