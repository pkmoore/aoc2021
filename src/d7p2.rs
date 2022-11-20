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
    let crabs: Vec<u64> = data.split(",").map(|x| x.parse::<u64>().unwrap()).collect();

    let (_, maybe_max) = crabs.iter().min_and_max();
    let max = *maybe_max.unwrap();

    let mut fuel_costs: Vec<u128> = Vec::new();
    fuel_costs.resize(max as usize, 0);

    // New rules, each step takes one more fuel
    // So going from 1 to 2 takes 1 fuel
    // Going from 1 to 3 takes 1 + 2 (=3) fuel
    // Increasing fuel costs form the sequence:
    // 0, 1, 3, 6, 10, 15, 21, 28...
    // These are the "triangular numbers" and a given
    // member of this sequence can be found by
    // (n * (n + 1)) / 2

    // These loops accumulate the cost for each crab to make it to
    // a given position.
    for crab_pos in crabs {
        for i in 0..max as usize {
            let difference: i128 = crab_pos as i128 - i as i128;
            let distance: u64 = difference.abs() as u64;
            fuel_costs[i] += distance as u128 * (distance as u128 + 1) / 2;
        }
    }

    let total_fuel = fuel_costs.iter().min().unwrap();
    println!("D7P2 Result: {}", total_fuel);

    Ok(())
}
