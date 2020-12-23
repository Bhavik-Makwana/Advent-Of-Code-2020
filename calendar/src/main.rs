extern crate time;
use time::PreciseTime;

mod day23;
mod readfile;

pub use crate::readfile::fileio;

fn main() {
    // let vec = fileio::read_file_batch(String::from("input/day23.txt"));
    let vec: String = String::from("463528179");
    let ans = day23::part_one(&vec);
    println!("part one: {:#?}", ans.unwrap());
    let start = PreciseTime::now();
    let ans2 = day23::part_two(&vec).unwrap();
    let end = PreciseTime::now();
    println!("part two: {:#?}", ans2);
    println!("took: {} seconds", start.to(end));
}
