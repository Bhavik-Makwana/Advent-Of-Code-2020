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
                vec.push(num);
            }
        }
    }
    let ans = part_one(&vec);
    println!("ans {:#?}", ans);
    let ans2 = part_two(&vec);
    println!("ans {:#?}", ans2);
}

//
//
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    //
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part_one(codes: & Vec<String>) -> i32 {
  let mut total = 0;
  for code in codes.iter() {
    let mut iter = code.split_whitespace();
    let mut range = iter.next().unwrap().split("-");
    let min: i32 = range.next().unwrap().parse().unwrap();
    let max: i32 = range.next().unwrap().parse().unwrap();
    let c = &iter.next().unwrap()[0..1].chars().next().unwrap();
    let pass = iter.next().unwrap();
    let mut count = 0;
    println!("{:?} {} {} {:?}",pass, min, max, c);
    
    for i in pass.chars() {
        if  i == *c {
            count += 1;
        }
    }
    if min <= count && count <= max {
        total += 1;
    }
  }
  total
}

fn part_two(codes: & Vec<String>) -> i32 {
    let mut total = 0;
    for code in codes.iter() {
        let mut iter = code.split_whitespace();
        let mut range = iter.next().unwrap().split("-");
        let loc_one: i32 = range.next().unwrap().parse::<i32>().unwrap()-1;
        let loc_two: i32 = range.next().unwrap().parse::<i32>().unwrap()-1;
        let c = &iter.next().unwrap()[0..1].chars().next().unwrap();
        let pass = iter.next().unwrap();
        let mut count = 0;
        // println!("{:?} {} {} {:?}",pass, min, max, c);
        if (pass.chars().nth(loc_one as usize).unwrap() == *c)
         ^ (pass.chars().nth(loc_two as usize).unwrap() == *c)  {
            total += 1;
        }
        
    }
    
    
  
  total
  }
