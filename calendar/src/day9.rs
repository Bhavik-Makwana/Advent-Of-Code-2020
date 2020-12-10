use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt::Error;
use std::iter::FromIterator;

pub fn part_one(ciphertext: &Vec<i64>) -> Result<i64, Error> {
    Ok(find_crash(&ciphertext).unwrap().1)
}

fn find_crash(ciphertext: &Vec<i64>) -> Result<(usize, i64), Error> {
    let queue_capacity = 25;
    let mut queue = VecDeque::with_capacity(queue_capacity);

    for i in 0..queue_capacity {
        queue.push_back(ciphertext[i]);
    }
    // &ciphertext[queue.len()..ciphertext.len()].iter().filter(|e| pair_exists(&queue, e)).
    for i in queue.len()..ciphertext.len() {
        if pair_exists(&queue, ciphertext[i]) {
            queue.pop_front();
            queue.push_back(ciphertext[i])
        } else {
            return Ok((i, ciphertext[i]));
        }
    }

    Ok((0, -1))
}

fn pair_exists(q: &VecDeque<i64>, val: i64) -> bool {
    let s: HashSet<&i64> = HashSet::from_iter(q.iter().clone());

    for i in (0..q.len()) {
        if s.contains(&(val - q[i])) {
            return true;
        }
    }
    false
}

pub fn part_two(ciphertext: &Vec<i64>) -> Result<i64, Error> {
    let (index, crash_val) = find_crash(&ciphertext).unwrap();

    for i in 0..index {
        let mut acc = ciphertext[i];
        for j in i + 1..index {
            acc += ciphertext[j];
            if acc == crash_val {
                let x = *(&ciphertext[i..j + 1].iter().min().unwrap())
                    + *(&ciphertext[i..j + 1].iter().max().unwrap());
                return Ok(x);
            }
            if acc > crash_val {
                break;
            }
        }
    }
    Ok(-1)
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
