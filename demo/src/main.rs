pub mod day1;

pub use crate::day1::solution;

fn main() {

    println!("Hello, world! {}", solution::hmm());
    let vec = Vec::new();
    vec.push(4);
    vec.push(3);
    solution::part_one(&vec);
    println!("Hello, world! {}", solution::hmm());
}
