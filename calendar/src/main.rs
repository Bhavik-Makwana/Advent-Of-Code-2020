extern crate time;
use time::PreciseTime;

mod day12;
mod readfile;

pub use crate::readfile::fileio;

fn main() {
    let vec = fileio::read_file(String::from("input/day12.txt"));

    let ans = day12::part_one(&vec);
    println!("part one: {:#?}", ans.unwrap());
    let start = PreciseTime::now();
    let ans2 = day12::part_two(&vec).unwrap();
    let end = PreciseTime::now();
    println!("part two: {:#?}", ans2);
    println!("took: {:?}s", start.to(end));
}
