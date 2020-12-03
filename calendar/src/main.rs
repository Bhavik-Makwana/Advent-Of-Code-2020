// mod day1;
mod day3;
mod readfile;

pub use crate::readfile::fileio;

fn main() {
    let vec = fileio::read_file_2d(String::from("input/day3.txt"));

    let ans = day3::part_one(&vec).unwrap();
    println!("part one: {:#?}", ans);
    let ans2 = day3::part_two(&vec).unwrap();
    println!("part two: {:#?}", ans2);
}
