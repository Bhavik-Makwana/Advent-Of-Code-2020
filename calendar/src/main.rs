// mod day1;
mod day6;
mod readfile;

pub use crate::readfile::fileio;

fn main() {
    let vec = fileio::read_file_batch(String::from("input/day6.txt"));

    let ans = day6::part_one(&vec);
    println!("part one: {:#?}", ans.unwrap());
    let ans2 = day6::part_two(&vec).unwrap();
    println!("part two: {:#?}", ans2);
}
