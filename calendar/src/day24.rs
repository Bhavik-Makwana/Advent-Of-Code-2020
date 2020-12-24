use std::collections::HashMap;
use std::fmt;
use std::fmt::Error;

#[derive(Hash, Eq, Debug, Clone)]
struct HexTile {
    e: i32,
    n: i32,
    // ne: i32,
    // nw: i32,
}
impl fmt::Display for HexTile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.e, self.n)
    }
}
impl PartialEq for HexTile {
    fn eq(&self, other: &Self) -> bool {
        self.e == other.e && self.n == other.n
        // && self.ne == other.ne
        // && self.nw == other.nw
    }
}

pub fn part_one(input: &Vec<String>) -> Result<i64, Error> {
    let total = create_grid(&input).iter().filter(|(_, v)| **v == true).count();
    Ok(total as i64)
}

fn create_grid(input: &Vec<String>) -> HashMap<HexTile, bool> {
    let mut tiles = HashMap::new();
    for line in input.iter() {
        let mut tile = HexTile { e: 0, n: 0 };
        let mut it = line.chars().peekable();
        while let Some(c) = it.next() {
            match c {
                's' => {
                    let x = it.next();
                    match x {
                        Some('e') => {
                            tile.n -= 1;
                            tile.e += 1;
                        }
                        Some('w') => {
                            tile.n -= 1;
                            tile.e -= 1;
                        }
                        _ => println!("no bueno"),
                    }
                }
                'e' => tile.e += 2,
                'w' => tile.e -= 2,
                'n' => {
                    let x = it.next();
                    match x {
                        Some('e') => {
                            tile.n += 1;
                            tile.e += 1;
                        }
                        Some('w') => {
                            tile.n += 1;
                            tile.e -= 1;
                        }
                        _ => println!("no bueno"),
                    }
                }
                _ => {
                    println!("nani");
                }
            }
        }
        if tiles.contains_key(&tile) {
            let new_val = !tiles.get(&tile).unwrap();
            tiles.insert(tile, new_val);
        } else {
            tiles.insert(tile, true);
        }
    }
    tiles
}

pub fn part_two(input: &Vec<String>) -> Result<i64, Error> {
    let mut tiles = create_grid(&input)
        .into_iter()
        .map(|(k, v)| if v { (k, '#') } else { (k, '.') })
        .collect::<HashMap<HexTile, char>>();
    let total = tiles.iter().filter(|(_, v)| **v == '#').count();
    let neighbours = vec![(2, 0), (-2, 0), (1, 1), (-1, 1), (-1, -1), (1, -1)];

    println!("Day 0: {}",  total);

    let mut new_state: HashMap<HexTile, char> = HashMap::new();
    for (tile, value) in tiles.iter() {
        new_state.insert(tile.clone(), *value);
        for neighbour in neighbours.iter() {
            let investigated_tile = HexTile {
                e: tile.e + neighbour.0,
                n: tile.n + neighbour.1,
            };
            if !tiles.contains_key(&investigated_tile) {
                new_state.insert(investigated_tile.clone(), '.');
            }
            
        }
    }
    tiles = new_state;

    
    for _ in 0..100 {
        let mut new_state: HashMap<HexTile, char> = HashMap::new();
        for (tile, value) in tiles.iter() {
            let mut adj = 0;
            for neighbour in neighbours.iter() {
                let investigated_tile = HexTile {
                    e: tile.e + neighbour.0,
                    n: tile.n + neighbour.1,
                };
                if tiles.contains_key(&investigated_tile) {
                    if *tiles.get(&investigated_tile).unwrap() == '#'
                        || *tiles.get(&investigated_tile).unwrap() == '?'
                    {
                        adj += 1;
                    }
                }
                else {
                    new_state.insert(investigated_tile.clone(), '.');
                }
            }
            if ((adj == 0 || adj > 2) && (*value == '#'))
                || (adj == 2 && (*value == '.'))
            {
                if (adj == 0 || adj > 2) && (*value == '#' ) {
                    new_state.insert(tile.clone(), '?');
                }
                if adj == 2 && (*value == '.' ) {
                    new_state.insert(tile.clone(), '!');
                }
            } else {
                new_state.insert(tile.clone(), *value);
            }
        }
        tiles = new_state.clone();

        tiles = tiles
            .into_iter()
            .map(|(k, v)| {
                if v == '?' {
                    (k, '.')
                } else if v == '!' {
                    (k, '#')
                } else {
                    (k, v)
                }
            })
            .collect::<HashMap<HexTile, char>>();

    }
    let total = tiles.iter().filter(|(_, v)| **v == '#').count();
    Ok(total as i64)
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_one_sample() {
        let vec = crate::readfile::fileio::read_file(String::from("input/test/day24.txt"));
        assert_eq!(crate::day24::part_one(&vec), Ok(10));
    }

    #[test]
    fn part_one_actual() {
        let vec = crate::readfile::fileio::read_file(String::from("input/day24.txt"));
        assert_eq!(crate::day24::part_one(&vec), Ok(512));
    }

    #[test]
    fn part_two_sample() {
        let vec = crate::readfile::fileio::read_file(String::from("input/test/day24.txt"));
        assert_eq!(crate::day24::part_two(&vec), Ok(2208));
    }

    #[test]
    fn part_two_actual() {
        let vec = crate::readfile::fileio::read_file(String::from("input/day24.txt"));
        assert_eq!(crate::day24::part_two(&vec), Ok(4120));
    }
}
