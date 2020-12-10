use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt::Error;
use std::iter::FromIterator;

pub fn part_one(input: &Vec<i64>) -> Result<i64, Error> {
    let mut prev = 0;
    let mut one = 0;
    let mut three = 0;
    let mut joltages = input.clone();
    joltages.sort();
    for joltage in joltages.iter() {
        if joltage - prev == 1 {
            prev = *joltage;
            one += 1;
        } else if joltage - prev == 3 {
            prev = *joltage;
            three += 1;
        }
    }
    three += 1;
    Ok(one * three)
}

pub fn part_two(voltages: &Vec<i64>) -> Result<i64, Error> {
    let mut joltages = voltages.clone();
    joltages.push(0);
    joltages.sort();

    Ok(dp_part_two(&joltages))
}
fn dp_part_two(joltages: &Vec<i64>) -> i64 {
    let n = joltages.len();
    let mut dp = vec![0; n];
    dp[0] = 1;
    dp[1] = 1;
    dp[2] = 2;
    for i in 3..joltages.len() {
        if joltages[i] - joltages[i - 1] <= 3 {
            dp[i] += dp[i - 1]
        }
        if joltages[i] - joltages[i - 2] <= 3 {
            dp[i] += dp[i - 2]
        }
        if joltages[i] - joltages[i - 3] <= 3 {
            dp[i] += dp[i - 3]
        }
    }
    dp[dp.len() - 1]
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
        let vec = crate::readfile::fileio::read_file_i64(String::from("input/day10.txt"));
        assert_eq!(crate::day10::part_one(&vec), Ok(220));
    }

    // #[test]
    // fn part_two_sample() {
    //     let vec = crate::readfile::fileio::read_file_int(String::from("input/test/day9.txt"));
    //     assert_eq!(crate::day9::part_two(&vec), Ok(62));
    // }

    #[test]
    fn part_two_actual() {
        let vec = crate::readfile::fileio::read_file_i64(String::from("input/day10.txt"));
        // let start_time = Utc::now().time();
        assert_eq!(crate::day10::part_two(&vec), Ok(19208));
        // let end_time = Utc::now().time();
        // let diff = end_time - start_time;
        // println!("time: {}", diff);
    }
}