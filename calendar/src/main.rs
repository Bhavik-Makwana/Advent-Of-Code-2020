// mod day1;
mod day1;
mod readfile;

pub use crate::readfile::fileio;

fn main() {
    let vec = fileio::read_file_int(String::from("input/day1.txt"));

    let ans = day1::part_one(&vec);
    println!("part one: {:#?}", ans);
    let ans2 = day1::part_two(&vec);
    println!("part two: {:#?}", ans2);
}
