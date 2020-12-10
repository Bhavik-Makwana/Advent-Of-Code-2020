use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Error;

pub fn part_one(input: &Vec<i64>) -> Result<i64, Error> {
    let mut joltages = input.clone();
    
    joltages.sort();
    let (one, three, _) = joltages.iter().fold((0, 0, 0), |(one, three, prev), jolt| match jolt-prev {
        1 => (one+1, three, *jolt),
        3 => (one, three+1, *jolt),
        _ => (one, three, prev),
    });

    Ok(one * (three+1))
}

pub fn part_two(input: &Vec<i64>) -> Result<i64, Error> {
    let mut joltages = input.clone();
    joltages.push(0);
    joltages.sort();

    Ok(dp_part_two(&joltages))

    // let lookup = HashSet::from_iter(joltages.iter().cloned());
    // let mut memo: HashMap<i64, i64> = HashMap::new();
    // let largest_val = joltages.iter().max().unwrap();
    // Ok(memoization_part_two(*largest_val, &mut memo, &lookup))
}

fn dp_part_two(joltages: &Vec<i64>) -> i64 {
    let n = joltages.len();
    let mut dp = vec![0; n];
    dp[0] = 1;
    dp[1] = 1;
    dp[2] = 2;
    for i in 3..joltages.len() {
        for j in 1..4 {
            if joltages[i] - joltages[i - j] <= 3 {
                dp[i] += dp[i - j]
            }
        }
        
    }
    dp[dp.len() - 1]
}

// Waaay slower than dp table
fn _memoization_part_two(jolt: i64, memo: &mut HashMap<i64, i64>, lookup: &HashSet<i64>) -> i64 {
    if lookup.contains(&jolt) {
        if jolt == 1 || jolt == 0 {
            return 1;
        } else if jolt == 2 {
            return 2;
        }
        if memo.contains_key(&jolt) {
            return *memo.get(&jolt).unwrap();
        }
        let res = _memoization_part_two(jolt - 3, memo, &lookup)
            + _memoization_part_two(jolt - 2, memo, &lookup)
            + _memoization_part_two(jolt - 1, memo, &lookup);
        memo.insert(jolt, res);
        return res;
    }
    0
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_one_sample() {
        let vec = crate::readfile::fileio::read_file_i64(String::from("input/test/day10.txt"));
        assert_eq!(crate::day10::part_one(&vec), Ok(220));
    }

    #[test]
    fn part_one_actual() {
        let vec = crate::readfile::fileio::read_file_i64(String::from("input/day10.txt"));
        assert_eq!(crate::day10::part_one(&vec), Ok(2210));
    }

    #[test]
    fn part_two_sample() {
        let vec = crate::readfile::fileio::read_file_i64(String::from("input/test/day10.txt"));
        assert_eq!(crate::day10::part_two(&vec), Ok(19208));
    }

    #[test]
    fn part_two_actual() {
        let vec = crate::readfile::fileio::read_file_i64(String::from("input/day10.txt"));
        // let start_time = Utc::now().time();
        assert_eq!(crate::day10::part_two(&vec), Ok(7086739046912));
        // let end_time = Utc::now().time();
        // let diff = end_time - start_time;
        // println!("time: {}", diff);
    }
}
