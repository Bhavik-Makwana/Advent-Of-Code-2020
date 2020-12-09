use std::fmt::Error;
use std::collections::VecDeque;

pub fn part_one(ciphertext: &Vec<i64>) -> Result<i64, Error> {
    let queue_capacity = 25;
    let mut queue = VecDeque::with_capacity(queue_capacity);
    
    for i in 0..queue_capacity {
        queue.push_back(ciphertext[i]);
    }

    for i in queue.len()..ciphertext.len() {
        if pair_exists(&queue, ciphertext[i]) {
            queue.pop_front();
            queue.push_back(ciphertext[i])
        }
        else {
            return Ok(ciphertext[i]);
        }
    }
    Ok(-1)
}

fn pair_exists(q: &VecDeque<i64>, val: i64) -> bool {
    for i in (0..q.len()) {
        for j in (i+1..q.len()) {
            if q[i] + q[j] == val {
                return true;
            }
        }
    }
    false
}

pub fn part_two(ciphertext: &Vec<i64>) -> Result<i64, Error> {
    let crash_val = part_one(&ciphertext).unwrap();
    let mut i = 0;
    let mut j = 0;
    while ciphertext[i] != crash_val {
        while ciphertext[j] != crash_val {
            if *(&ciphertext[i..j+1].iter().sum::<i64>()) == crash_val {
                let x = *(&ciphertext[i..j+1].iter().min().unwrap()) + *(&ciphertext[i..j+1].iter().max().unwrap());
                return Ok(x);
            }
            j += 1;
        }
        i += 1;
        j = i;
    }
    Ok(2)
}


#[cfg(test)]
mod tests {
    // #[test]
    // fn part_one_sample() {
    //     let vec = crate::readfile::fileio::read_file_int(String::from("input/test/day9.txt"));
    //     assert_eq!(crate::day9::part_one(&vec), Ok(127));
    // }

    #[test]
    fn part_one_actual() {
        let vec = crate::readfile::fileio::read_file_i64(String::from("input/day9.txt"));
        assert_eq!(crate::day9::part_one(&vec), Ok(542529149));
    }

    // #[test]
    // fn part_two_sample() {
    //     let vec = crate::readfile::fileio::read_file_int(String::from("input/test/day9.txt"));
    //     assert_eq!(crate::day9::part_two(&vec), Ok(62));
    // }

    #[test]
    fn part_two_actual() {
        let vec = crate::readfile::fileio::read_file_i64(String::from("input/day9.txt"));
        assert_eq!(crate::day9::part_two(&vec), Ok(75678618));
    }
}
