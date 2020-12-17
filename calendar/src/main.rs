extern crate time;
use time::PreciseTime;

mod day17;
mod readfile;

pub use crate::readfile::fileio;

fn main() {
    let vec = fileio::read_file_2d(String::from("input/day17.txt"));

    let ans = day17::part_one(&vec);
    println!("part one: {:#?}", ans.unwrap());
    let start = PreciseTime::now();
    let ans2 = day17::part_two(&vec).unwrap();
    let end = PreciseTime::now();
    println!("part two: {:#?}", ans2);
    println!("took: {} seconds", start.to(end));
}
