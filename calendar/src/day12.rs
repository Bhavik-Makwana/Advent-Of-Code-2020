use std::fmt::Error;

struct Ship {
    east: i32,
    north: i32,
    angle: i32,
}

struct Waypoint {
    east: i32,
    north: i32
}

pub fn part_one(input: &Vec<String>) -> Result<i32, Error> {
    let mut ship = Ship { east: 0, north: 0, angle: 90};
    
    for direction in input.iter() {
        let (action, amount) = direction.split_at(1);
        let amount = amount.parse::<i32>().unwrap();
        match action {
            "N" => ship.north += amount,
            "E" => ship.east += amount,
            "S" => ship.north -= amount,
            "W" => ship.east -= amount,
            "F" => {
                match ship.angle {
                    0 => ship.north += amount,
                    90 => ship.east += amount,
                    180 => ship.north -= amount,
                    270 => ship.east -= amount,
                    _ => (),
                }
            }
            "L" => ship.angle = (ship.angle - amount).rem_euclid(360),
            "R" => ship.angle = (ship.angle + amount).rem_euclid(360),
            _ => (),
        }
    }

    Ok(ship.north.abs() + ship.east.abs())
}


pub fn part_two(input: &Vec<String>) -> Result<i32, Error> {
    let mut waypoint = Waypoint { east: 10, north: 1};
    let mut ship = Ship { east: 0, north: 0, angle: 0};

    for direction in input.iter() {
        let (action, amount) = direction.split_at(1);
        let amount = amount.parse::<i32>().unwrap();
        
        match action {
            "F" => {
                ship.east += waypoint.east * amount;
                ship.north += waypoint.north * amount;
            },
            "N" => waypoint.north += amount,
            "E" => waypoint.east += amount,
            "S" => waypoint.north -= amount,
            "W" => waypoint.east -= amount,
            _ => {
                match &direction[..] {
                    "L180" | "R180" => {
                        waypoint.east = -waypoint.east;
                        waypoint.north = -waypoint.north;
                    },
                    "L90" | "R270" => {
                        let temp = waypoint.east;
                        waypoint.east = -waypoint.north;
                        waypoint.north = temp;
                    }
                    "R90" | "L270" => {
                        let temp = waypoint.east;
                        waypoint.east = waypoint.north;
                        waypoint.north = -temp;
                    }
                    _ => (),
                }
            }
        }
    }
    Ok(ship.east.abs() + ship.north.abs())
}


#[cfg(test)]
mod tests {

    #[test]
    fn part_one_sample() {
        let vec = crate::readfile::fileio::read_file(String::from("input/test/day12.txt"));
        assert_eq!(crate::day12::part_one(&vec), Ok(25));
    }

    #[test]
    fn part_one_actual() {
        let vec = crate::readfile::fileio::read_file(String::from("input/day12.txt"));
        assert_eq!(crate::day12::part_one(&vec), Ok(1186));
    }

    #[test]
    fn part_two_sample() {
        let vec = crate::readfile::fileio::read_file(String::from("input/test/day12.txt"));
        assert_eq!(crate::day12::part_two(&vec), Ok(286));
    }

    #[test]
    fn part_two_actual() {
        let vec = crate::readfile::fileio::read_file(String::from("input/day12.txt"));
        // let start_time = Utc::now().time();
        assert_eq!(crate::day12::part_two(&vec), Ok(47806));
        // let end_time = Utc::now().time();
        // let diff = end_time - start_time;
        // println!("time: {}", diff);
    }
}
