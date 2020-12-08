// mod day1;
mod day8;
mod readfile;

pub use crate::readfile::fileio;

fn main() {
    let vec = fileio::read_file(String::from("input/day8.txt"));

    let ans = day8::part_one(&vec);
    println!("part one: {:#?}", ans.unwrap());
    let ans2 = day8::part_two(&vec).unwrap();
    println!("part two: {:#?}", ans2);
}
