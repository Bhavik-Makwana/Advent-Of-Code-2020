use std::collections::HashSet;
use std::fmt::Error;
use std::iter::FromIterator;

pub fn part_one(years: &Vec<i32>) -> Result<i32, Error> {
    let s: HashSet<i32> = HashSet::from_iter(years.iter().cloned());
    Ok(years
        .into_iter()
        .filter(|year| s.contains(&(2020 - *year)))
        .product())
}

pub fn part_two(years: &Vec<i32>) -> Result<i32, Error> {
    let s: HashSet<i32> = HashSet::from_iter(years.iter().cloned());

    for i in 0..s.len() {
        for j in i..s.len() {
            let x = 2020 - years[i] - years[j];
            if s.contains(&x) {
                return Ok(x * years[i] * years[j]);
            }
        }
    }

    Err(Error)
}
