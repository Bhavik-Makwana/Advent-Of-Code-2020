extern crate time;
use time::PreciseTime;

mod day22;
mod readfile;

pub use crate::readfile::fileio;

fn main() {
    let vec = fileio::read_file_batch(String::from("input/day22.txt"));

    let ans = day22::part_one(&vec);
    println!("part one: {:#?}", ans.unwrap());
    let start = PreciseTime::now();
    let ans2 = day22::part_two(&vec).unwrap();
    let end = PreciseTime::now();
    println!("part two: {:#?}", ans2);
    println!("took: {} seconds", start.to(end));
}
