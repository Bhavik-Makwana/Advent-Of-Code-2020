extern crate time;
use time::PreciseTime;

mod day20;
mod readfile;

pub use crate::readfile::fileio;

fn main() {
    let vec = fileio::read_file_batch(String::from("input/day20.txt"));

    let ans = day20::part_one(&vec);
    println!("part one: {:#?}", ans.unwrap());
    let start = PreciseTime::now();
    let ans2 = day20::part_two(&vec).unwrap();
    let end = PreciseTime::now();
    println!("part two: {:#?}", ans2);
    println!("took: {} seconds", start.to(end));
}
  