use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::iter::FromIterator;
fn main() {
    //
    let mut vec = Vec::new();
    if let Ok(lines) = read_lines("in.txt") {
        //
        for line in lines {
            if let Ok(num) = line {
                vec.push(num.parse::<i32>().unwrap());
            }
        }
    }
    let ans = part_two(&vec);
    println!("ans {}", ans);
}

//
//
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    //
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part_one(years: & Vec<i32>) -> i32 {
  let s: HashSet<i32> = HashSet::from_iter(years.iter().cloned());
  for year in years.iter() {
      let x = 2020-year;
      if (s.contains(&x)) {
          return year * (2020-year);
      }
  }
  -1
}

fn part_two(years: & Vec<i32>) -> i32 {
    let s: HashSet<i32> = HashSet::from_iter(years.iter().cloned());
    for i in 0..s.len() {
        for j in i..s.len() {
            let x = 2020-years[i]-years[j];
            if (s.contains(&x)) {
                return x * years[i] * years[j];
            }
        }    
    }
    -1
  }
