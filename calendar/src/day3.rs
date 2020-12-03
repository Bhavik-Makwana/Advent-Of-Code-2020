use std::fmt::Error;
pub fn part_one(map: &Vec<Vec<char>>) -> Result<i64, Error> {
    helper(map, 3, 1)
}

pub fn part_two(map: &Vec<Vec<char>>) -> Result<i64, Error> {
    let arr: [(usize, usize); 5] = [(1,1), (3,1), (5,1), (7,1), (1,2)];
    arr.iter().map(|(right, down)| helper(map, *right, *down))
    .product()
}

fn helper(map: &Vec<Vec<char>>, r: usize, d: usize) -> Result<i64, Error> { 
    let m = map[0].len();
    let n = map.len();

    Ok((0..n)
    .filter(|i| map[(*i*d) % n][(*i*r) % m] == '#' && *i*d < n )
    .count() as i64)
}