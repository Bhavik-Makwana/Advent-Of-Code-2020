use std::collections::HashSet;
use std::iter::FromIterator;

pub fn part_one(years: &Vec<i32>) -> i32 {
    let s: HashSet<i32> = HashSet::from_iter(years.iter().cloned());
    for year in years.iter() {
        let x = 2020 - year;
        if s.contains(&x) {
            return year * (2020 - year);
        }
    }
    -1
}

pub fn part_two(years: &Vec<i32>) -> i32 {
    let s: HashSet<i32> = HashSet::from_iter(years.iter().cloned());
    for i in 0..s.len() {
        for j in i..s.len() {
            let x = 2020 - years[i] - years[j];
            if s.contains(&x) {
                return x * years[i] * years[j];
            }
        }
    }
    -1
}
