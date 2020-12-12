use std::fmt::Error;

pub fn part_one(codes: &Vec<String>) -> Result<i32, Error> {
    let mut total = 0;
    for code in codes.iter() {
        let mut iter = code.split_whitespace();
        let mut range = iter.next().unwrap().split("-");
        let min: i32 = range.next().unwrap().parse().unwrap();
        let max: i32 = range.next().unwrap().parse().unwrap();
        let c = &iter.next().unwrap()[0..1].chars().next().unwrap();
        let pass = iter.next().unwrap();

        let count = pass.chars().filter(|i| i == c).count() as i32;

        if min <= count && count <= max {
            total += 1;
        }
    }
    Ok(total)
}

pub fn part_two(codes: &Vec<String>) -> Result<i32, Error> {
    let mut total = 0;
    for code in codes.iter() {
        let mut iter = code.split_whitespace();
        let mut range = iter.next().unwrap().split("-");
        let loc_one: i32 = range.next().unwrap().parse::<i32>().unwrap() - 1;
        let loc_two: i32 = range.next().unwrap().parse::<i32>().unwrap() - 1;
        let c = &iter.next().unwrap()[0..1].chars().next().unwrap();
        let pass = iter.next().unwrap();

        if (pass.chars().nth(loc_one as usize).unwrap() == *c)
            ^ (pass.chars().nth(loc_two as usize).unwrap() == *c)
        {
            total += 1;
        }
    }

    Ok(total)
}
