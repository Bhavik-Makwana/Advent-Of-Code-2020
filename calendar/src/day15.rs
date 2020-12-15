use std::collections::HashMap;
use std::fmt::Error;

pub fn part_one(input: &Vec<String>) -> Result<i32, Error> {
    let start = input[0].split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();    
    Ok(memory_game(&start, 2020))
}



pub fn part_two(input: &Vec<String>) -> Result<i32, Error> {
    let start = input[0].split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();    
    Ok(memory_game_array(&start, 30000000) as i32)
}

// found online
fn memory_game_array(start: &Vec<usize>, limit: usize) -> usize {
    let mut memory: Vec<usize> = vec![usize::MAX; limit];
    start.iter().enumerate().for_each(|(i, &x)| memory[x] = i);
    
    let mut last_spoken = *start.iter().last().unwrap();
    for turn in start.len() - 1..limit - 1 {
        let mut loc = turn;
        std::mem::swap(&mut loc, &mut memory[last_spoken]); 
        last_spoken = turn.saturating_sub(loc); 
    }
    last_spoken
}

fn memory_game(start: &Vec<i32>, limit: i32) -> i32 {
    let mut memory: HashMap<i32, i32> = HashMap::new();
    let mut turn = 1;
    let mut last_spoken = 0;
    for i in start.iter() {
        memory.insert(*i, turn);
        last_spoken = *i;
        turn += 1;
    }
    let mut previous: i32 = *memory.get(&last_spoken).unwrap();
    println!("{:?}", memory);
    while turn <= limit {
        if memory.contains_key(&last_spoken) && previous != 0 {
            last_spoken = turn - 1 - previous;
        }
        else {
            last_spoken = 0;
        }

        if !memory.contains_key(&last_spoken) {
            previous = 0;
        }
        else {
            previous = *memory.get(&last_spoken).unwrap();
        }
        memory.insert(last_spoken, turn);
        turn +=1 
    }
    last_spoken
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_one_sample() {
        let vec = crate::readfile::fileio::read_file(String::from("input/test/day15.txt"));
        assert_eq!(crate::day15::part_one(&vec), Ok(436));
    }

    #[test]
    fn part_one_actual() {
        let vec = crate::readfile::fileio::read_file(String::from("input/day15.txt"));
        assert_eq!(crate::day15::part_one(&vec), Ok(421));
    }

    #[test]
    fn part_two_sample() {
        let vec = crate::readfile::fileio::read_file(String::from("input/test/day15.txt"));
        assert_eq!(crate::day15::part_two(&vec), Ok(175594));
    }

    #[test]
    fn part_two_actual() {
        let vec = crate::readfile::fileio::read_file(String::from("input/day15.txt"));
        assert_eq!(crate::day15::part_two(&vec), Ok(436));
    }
}
