use std::collections::HashSet;
use std::iter::FromIterator;
use std::fmt::Error;

pub fn part_one(years: &Vec<i32>) -> Result<i32, Error> {
    let s: HashSet<i32> = HashSet::from_iter(years.iter().cloned());
    let vec: Vec<&i32> = years.into_iter().filter(|year| s.contains(&(2020-*year))).collect::<Vec<_>>();
    Ok(vec[0] * vec[1])
}

pub fn part_two(years: &Vec<i32>) -> Result<i32, Error> {
    let s: HashSet<i32> = HashSet::from_iter(years.iter().cloned());
    for i in 0..s.len() {
        for j in i..s.len() {
            let x = 2020 - years[i] - years[j];
            if s.contains(&x) {
                return Ok(x * years[i] * years[j])
            }
        }
    }
    Err(Error)
}
