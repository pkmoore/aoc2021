use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

// I got a little carried away with the type system here.
// Didn't actually need this in the end
trait MinAndMaxable<T: Ord> {
    fn min_and_max(self) -> (Option<T>, Option<T>);
}

impl<T: Ord + Copy, I: Iterator<Item = T>> MinAndMaxable<T> for I
where
    I: Iterator,
    T: Ord + Copy,
{
    fn min_and_max(self) -> (Option<T>, Option<T>) {
        let mut min: Option<T> = None;
        let mut max: Option<T> = None;
        for a in self {
            min = min.map_or_else(|| Some(a), |i| Some(std::cmp::min(i, a)));
            max = max.map_or_else(|| Some(a), |i| Some(std::cmp::max(i, a)));
        }
        (min, max)
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("./data/day7.txt")?;
    let mut reader = BufReader::new(file);
    let mut data = String::new();
    reader.read_to_string(&mut data)?;
    let mut crabs: Vec<u32> = data.split(",").map(|x| x.parse::<u32>().unwrap()).collect();

    let l = crabs.len();

    // This function will ensure the middle element of the array is the correct value
    // This is a cheap way of finding the median without sorting the whole thing
    crabs.select_nth_unstable(l / 2);
    let dest = crabs[l / 2];

    let total_fuel: i32 = crabs
        .iter()
        // For each crab, find the distance from the destination (median)
        // by subtracting and taking the absolute value
        .map(|x| ((*x as i32) - (dest as i32) as i32).abs())
        // Sum up all the distances to get our total fuel
        .sum();

    println!("D7P1 Result: {}", total_fuel);

    Ok(())
}
