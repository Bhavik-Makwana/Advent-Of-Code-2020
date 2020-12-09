// mod day1;
mod day9;
mod readfile;

pub use crate::readfile::fileio;

fn main() {
    let vec = fileio::read_file_i64(String::from("input/day9.txt"));

    let ans = day9::part_one(&vec);
    println!("part one: {:#?}", ans.unwrap());
    let ans2 = day9::part_two(&vec).unwrap();
    println!("part two: {:#?}", ans2);
}
