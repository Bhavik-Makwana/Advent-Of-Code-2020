// mod day1;
extern crate time;
use time::PreciseTime;

mod day10;
mod readfile;

pub use crate::readfile::fileio;

fn main() {
    let vec = fileio::read_file_i64(String::from("input/day10.txt"));

    let ans = day10::part_one(&vec);
    println!("part one: {:#?}", ans.unwrap());
    let start = PreciseTime::now();
    let ans2 = day10::part_two(&vec).unwrap();
    let end = PreciseTime::now();
    println!("part two: {:#?}", ans2);
    println!("took: {:?}s", start.to(end));
}
