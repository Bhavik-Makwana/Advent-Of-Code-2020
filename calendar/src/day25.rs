use std::collections::HashMap;
use std::fmt;
use std::fmt::Error;



pub fn part_one(input: &Vec<String>) -> Result<i64, Error> {
    let card_pk = input[0].parse::<i64>().unwrap();
    let mut door_pk = input[1].parse::<i64>().unwrap();
    println!("cpk {}", card_pk);
    println!("dpk {}", door_pk);
    let subject_number = 7;
    let mut loop_size = 1;
    let mut transformed = 1;
    loop {
        transformed *= subject_number;
        transformed = transformed % 20201227;
  
        // println!("subject num {}", transformed);
        if transformed == card_pk {
            break;
        }
        loop_size += 1;
    }
    println!(" loop size  {}", loop_size);
    let mut transformed = 1;
    for i in 0..loop_size {
        transformed *= door_pk;
        transformed = transformed % 20201227;
    }
    println!("enc key {}", transformed);
    Ok(door_pk)
}


pub fn part_two(input: &Vec<String>) -> Result<i64, Error> {
    
    Ok(2)
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_one_sample() {
        let vec = crate::readfile::fileio::read_file(String::from("input/test/day25.txt"));
        assert_eq!(crate::day25::part_one(&vec), Ok(14897079));
    }

    // #[test]
    // fn part_one_actual() {
    //     let vec = crate::readfile::fileio::read_file(String::from("input/day25.txt"));
    //     assert_eq!(crate::day25::part_one(&vec), Ok(512));
    // }

    // #[test]
    // fn part_two_sample() {
    //     let vec = crate::readfile::fileio::read_file(String::from("input/test/day25.txt"));
    //     assert_eq!(crate::day25::part_two(&vec), Ok(2208));
    // }

    // #[test]
    // fn part_two_actual() {
    //     let vec = crate::readfile::fileio::read_file(String::from("input/day25.txt"));
    //     assert_eq!(crate::day25::part_two(&vec), Ok(4120));
    // }
}
