// mod day1;
mod day4;
mod readfile;

pub use crate::readfile::fileio;

fn main() {
    let vec = fileio::read_file_batch(String::from("input/day4.txt"));

    let ans = day4::part_one(&vec);
    println!("part one: {:#?}", ans.unwrap());
    let ans2 = day4::part_two(&vec).unwrap();
    println!("part two: {:#?}", ans2);
}
