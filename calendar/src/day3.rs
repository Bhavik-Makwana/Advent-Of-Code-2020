use std::fmt::Error;
pub fn part_one(map: &Vec<Vec<char>>) -> Result<i64, Error> {
    helper(map, 3, 1)
}

pub fn part_two(map: &Vec<Vec<char>>) -> Result<i64, Error> {
    Ok(helper(map, 1, 1).unwrap() * helper(map, 3, 1).unwrap() 
    * helper(map, 5, 1).unwrap() * helper(map, 7, 1).unwrap()
    * helper(map, 1, 2).unwrap())
}

fn helper(map: &Vec<Vec<char>>, r: i32, d: i32) -> Result<i64, Error> {
    let mut total = 0;
    let mut right = 0;
    let mut down = 0;
    let m = map[0].len();
    let n = map.len();
    while down < n {
        if map[down][right] == '#' {
            total += 1;
        }
        down = (down + d as usize);
        right = (right + r as usize) % m;
    }
    Ok(total)
}