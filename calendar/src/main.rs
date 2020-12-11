extern crate time;
use time::PreciseTime;

mod day11;
mod readfile;

pub use crate::readfile::fileio;

fn main() {
    let vec = fileio::read_file_2d(String::from("input/day11.txt"));

    let ans = day11::part_one(&vec);
    println!("part one: {:#?}", ans.unwrap());
    let start = PreciseTime::now();
    let ans2 = day11::part_two(&vec).unwrap();
    let end = PreciseTime::now();
    println!("part two: {:#?}", ans2);
    println!("took: {:?}s", start.to(end));
}
