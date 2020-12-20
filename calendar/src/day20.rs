use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Error;
use itertools::Itertools; // 0.8.2
#[derive(Debug, Default, Clone)]
struct Tile {
    id: i64,
    image: Vec<Vec<char>>,
}


pub fn part_one(input: &Vec<Vec<String>>) -> Result<i64, Error> {
    let all_edges = HashSet::new();
    let re = Regex::new(r"[^0-9.]").unwrap();
    let mut tiles = Vec::new();
    let mut just_tiles = Vec::new();
    for tile in input.iter() {
        let mut curr : Tile = Tile{id: 0, image: Vec::<Vec<char>>::new()};
        let mut c = Vec::new();
        for (i, line) in tile.iter().enumerate() {
            if i == 0 {
                let x = re.replace_all(line, "");
                curr.id = x.parse::<i64>().unwrap();
            }
            else {
                curr.image.push(line.chars().collect::<Vec<char>>());
                c.push(line.chars().collect::<Vec<char>>());
            }
        }
        tiles.push(curr);
        just_tiles.push(c);
    }
    // for row in tiles[0].image.iter() {
    //     println!("{:?}", row);
    // }
    // // rotate_90_cw(&mut tiles[0]);
    // flip_matrix(&mut tiles[0], false);
    // flip_matrix(&mut tiles[0], true);
    // flip_matrix(&mut tiles[0], true);
    // flip_matrix(&mut tiles[0], false);
    // println!("");
    // for row in tiles[0].image.iter() {
    //     println!("{:?}", row);
    // }
    // println!("");
    for tile in tiles.iter() {
        tile.image[0]
    }
    let mut prod: i64 = 1;
    for i in 0..tiles.len() {
        let mut count = 0;
        for j in 0..tiles.len() {
            if i != j {
                count += compare_edges(&mut tiles, i, j) 
            }
        }
        // println!("count {}", count);
        if count == 2 {
            // println!("{}", tiles[i].id);
            prod *= tiles[i].id;
        }
    }
    // for perm in just_tiles.iter().permutations(just_tiles.len()).unique() {
    //     for row in perm.iter() {
    //         println!("{:?}", row);
    //     }
    // }
    Ok(prod) // 156
}

fn compare_edges(tiles: &mut Vec<Tile>, i: usize, j: usize ) -> i32 {
    // println!("Tile 1: {}", tiles[i].id);
    
    let mut count = 0 ;
    for _ in 0..4 {
        rotate_90_cw(&mut tiles[j]);

        let mut matching = false;
        for _ in 0..4 {
            
            rotate_90_cw(&mut tiles[i]);
            let mut c_matching = true;
            for k in 0..tiles[i].image[0].len() {
                if tiles[j].image[0][k] != tiles[i].image[0][k] {
                    c_matching = false;
                }
            }
            
            if c_matching {
                matching = true;
            }
            c_matching = true;
            
            flip_matrix(&mut tiles[i], false);
            for k in 0..tiles[i].image[0].len() {
                if tiles[j].image[0][k] != tiles[i].image[0][k] {
                    c_matching = false;
                }
            }
            flip_matrix(&mut tiles[i], false);

            if c_matching {
                matching = true;
            }
            c_matching = true;

            flip_matrix(&mut tiles[i], true);
            for k in 0..tiles[i].image[0].len() {
                if tiles[j].image[0][k] != tiles[i].image[0][k] {
                    c_matching = false;
                }
            }
            flip_matrix(&mut tiles[i], true);

            if c_matching {
                matching = true;
            }
            c_matching = true;

            flip_matrix(&mut tiles[i], false);
            flip_matrix(&mut tiles[i], true);
            for k in 0..tiles[i].image[0].len() {
                if tiles[j].image[0][k] != tiles[i].image[0][k] {
                    c_matching = false;
                }
            }
            flip_matrix(&mut tiles[i], true);
            flip_matrix(&mut tiles[i], false);

            if c_matching {
                matching = true;
            }


            
        }
        if matching {
            // println!("{}", count);
            count += 1;
        }
    }
    // println!("ret {}", count);
        count
}

fn rotate_90_cw(tile: &mut Tile) {
    let n = 10;
    for i in 0..n/2 {
        for j in i..n-i-1 {
            let temp = tile.image[i][j];
            tile.image[i][j] = tile.image[j][n-1-i];
            tile.image[j][n-1-i] = tile.image[n-1-i][n-1-j];
            tile.image[n-1-i][n-1-j] = tile.image[n-1-j][i];
            tile.image[n-1-j][i] = temp;
        }
    }
}

fn flip_matrix(tile: &mut Tile, h: bool) {
    if h {
        for i in 0..tile.image.len() {
            let mut x = Vec::new();
            for j in (0..tile.image[0].len()).rev() {
                // println!("{}", tile.image[i][j]);
                x.push(tile.image[i][j]);
            }
            // println!("{:?}", x);
            for j in 0..tile.image.len() {
                tile.image[i][j] = x[j];
            }
        }
     
    }
    else {
        // for row in tile.image.iter() {
        //     println!("{:?}", row);
        // }
        // println!("");
        for i in 0..tile.image[0].len() {
            let mut x = Vec::new();
            for j in (0..tile.image.len()).rev() {
                x.push(tile.image[j][i]);
            }
            for j in 0..tile.image.len() {
                tile.image[j][i] = x[j];
            }
        }
        // for row in tile.image.iter() {
        //     println!("{:?}", row);
        // }
    }
}

pub fn part_two(input: &Vec<Vec<String>>) -> Result<i32, Error> {
    Ok(2) // 363
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_one_sample() {
        let vec = crate::readfile::fileio::read_file_batch(String::from("input/test/day20.txt"));
        assert_eq!(crate::day20::part_one(&vec), Ok(20899048083289));
    }

    // #[test]
    // fn part_one_actual() {
    //     let vec = crate::readfile::fileio::read_file_batch(String::from("input/day20.txt"));
    //     assert_eq!(crate::day20::part_one(&vec), Ok(276));
    // }

    // #[test]
    // fn part_two_sample() {
    //     let vec = crate::readfile::fileio::read_file_batch(String::from("input/test/day20.txt"));
    //     assert_eq!(crate::day20::part_two(&vec), Ok(848));
    // }

    // #[test]
    // fn part_two_actual() {
    //     let vec = crate::readfile::fileio::read_file_batch(String::from("input/day20.txt"));
    //     assert_eq!(crate::day20::part_two(&vec), Ok(2136));
    // }
}
