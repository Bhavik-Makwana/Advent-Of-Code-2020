use std::fmt::Error;

// enum direction {
//     NORTH(i32),

// }
pub fn part_one(input: &Vec<String>) -> Result<i32, Error> {
    let mut north: i32 = 0;
    let mut east: i32 = 0;
    let mut angle: i32 = 90;
    
    for direction in input.iter() {
        let (action, amount) = direction.split_at(1);
        let amount = amount.parse::<i32>().unwrap();
        match action {
            "N" => north += amount,
            "E" => east += amount,
            "S" => north -= amount,
            "W" => east -= amount,
            "F" => {
                match angle {
                    0 => north += amount,
                    90 => east += amount,
                    180 => north -= amount,
                    270 => east -= amount,
                    _ => (),
                }
            }
            "L" => angle = (angle - amount).rem_euclid(360),
            "R" => angle = (angle + amount).rem_euclid(360),
            _ => (),
        }
    }
    println!("{} {}", east, north);
    println!("{}", -180 % 360);

    Ok(north.abs() + east.abs())
}

pub fn part_two(input: &Vec<String>) -> Result<i32, Error> {
    let mut waypoint_coords: Vec<i32> = vec![10, 1];
    let mut ship_coords: Vec<i32> = vec![0, 0];

    for direction in input.iter() {
        let (action, amount) = direction.split_at(1);
        let amount = amount.parse::<i32>().unwrap();
        
        match action {
            "F" => {
                ship_coords[0] += waypoint_coords[0] * amount;
                ship_coords[1] += waypoint_coords[1] * amount;
            },
            "N" => waypoint_coords[1] += amount,
            "E" => waypoint_coords[0] += amount,
            "S" => waypoint_coords[1] -= amount,
            "W" => waypoint_coords[0] -= amount,
            _ => {
                match &direction[..] {
                    "L180" | "R180" => {
                        waypoint_coords[0] = -waypoint_coords[0];
                        waypoint_coords[1] = -waypoint_coords[1];
                    },
                    "L90" | "R270" => {
                        let temp = waypoint_coords[0];
                        waypoint_coords[0] = -waypoint_coords[1];
                        waypoint_coords[1] = temp;
                    }
                    "R90" | "L270" => {
                        let temp = waypoint_coords[0];
                        waypoint_coords[0] = waypoint_coords[1];
                        waypoint_coords[1] = -temp;
                    }
                    _ => (),
                }
            }
        }
    }
    Ok(ship_coords[0].abs() + ship_coords[1].abs())
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
