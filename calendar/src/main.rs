// mod day1;
mod day5;
mod readfile;

pub use crate::readfile::fileio;

fn main() {
    let vec = fileio::read_file(String::from("input/day5.txt"));

    let ans = day5::part_one(&vec);
    println!("part one: {:#?}", ans.unwrap());
    let ans2 = day5::part_two(&vec).unwrap();
    println!("part two: {:#?}", ans2);
}
