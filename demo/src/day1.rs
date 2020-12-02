use std::collections::HashSet;

pub mod solution {
    pub fn hmm() -> i32{
         200*2
    }


    pub fn part_one(years: &Vec<i32>) -> i32 {
        let s: HashSet<i32> = HashSet::from_iter(years.iter().cloned());
        for year in years.iter() {
            let x = 2020 - year;
            if (s.contains(&x)) {
                return year * (2020 - year);
            }
        }
        -1
    }
}