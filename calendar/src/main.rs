// mod day1;
mod day7;
mod readfile;

pub use crate::readfile::fileio;

fn main() {
    let vec = fileio::read_file(String::from("input/day7.txt"));

    let ans = day7::part_one(&vec);
    println!("part one: {:#?}", ans.unwrap());
    let ans2 = day7::part_two(&vec).unwrap();
    println!("part two: {:#?}", ans2);
}
