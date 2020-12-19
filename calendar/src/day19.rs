use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Error;

pub fn part_one(input: &Vec<String>) -> Result<i32, Error> {
    let mut grammar: HashMap<String, Vec<Vec<String>>> = HashMap::new();
    let mut test_string = Vec::new();
    let mut terminals = HashSet::new();

    let mut f = false;
    for line in input.iter() {
        if line.len() == 0 {
            f = true;
        }
        if f {
            if line != "" {
                test_string.push(line);
            }
        } else {
            let mut iter = line.split(":");
            let lhs = iter.next().unwrap();
            let mut rhs = Vec::new();
            let mut iter = iter.next().unwrap().split("|").peekable();
            while let Some(&c) = iter.peek() {
                let mut x = c.split(" ");
                let mut v = Vec::new();
                for i in x {
                    if i != "" {
                        v.push(String::from(i));
                    }
                    for k in i.chars() {
                        if k.is_alphabetic() {
                            terminals.insert(String::from(k));
                        }
                    }
                }
                rhs.push(v);
                iter.next();
            }
            grammar.insert(String::from(lhs), rhs);
        }
    }
    let mut total = 0;
    for string in test_string.iter() {
        if cyk(string, &grammar, &terminals) {
            total += 1;
        }
    }
    println!("{:?}", total);
    Ok(total) // 156
}

fn cyk(
    string: &String,
    grammar: &HashMap<String, Vec<Vec<String>>>,
    terminals: &HashSet<String>,
) -> bool {
    let n = string.len() ;
    let mut P = vec![vec![HashSet::new(); n]; n];
    for s in 0..n {
        for (key, value) in grammar.iter() {
            if value.len() == 1 {
                if (value[0][0] == "a" && string.chars().nth(s).unwrap() == 'a')
                    || (value[0][0] == "b" && string.chars().nth(s).unwrap() == 'b')
                {
                    P[0][s].insert(key);
                }
            }
        }
    }

    for l in 1..n  {
        // row
        for s in 0..n - l  {
            // col in the row
            for p in 0..l  {
                //  previous
                for (key, value) in grammar.iter() {
                    for v in value.iter() {
                        // R_a -> R_b R_c
                        if v.len() == 2 {
                            if P[p][s].contains(&v[0]) && P[l - p-1 ][s + p+1].contains(&v[1]) {
                                P[l][s].insert(key);
                            }
                        }
                    }
                }
            }
        }
    }

    // for i in P.iter().rev() {
    //     println!("{:?}", i);
    // }
    if P[n-1][0].len() > 0 {
        println!("{}", string);
        return true;
    }
    false
}



pub fn part_two(input: &Vec<String>) -> Result<i32, Error> {
    Ok(2) // 363
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_one_sample() {
        let vec = crate::readfile::fileio::read_file(String::from("input/test/day19.txt"));
        assert_eq!(crate::day19::part_one(&vec), Ok(2));
    }

    // #[test]
    // fn part_one_actual() {
    //     let vec = crate::readfile::fileio::read_file(String::from("input/day19.txt"));
    //     assert_eq!(crate::day19::part_one(&vec), Ok(276));
    // }

    // #[test]
    // fn part_two_sample() {
    //     let vec = crate::readfile::fileio::read_file(String::from("input/test/day19.txt"));
    //     assert_eq!(crate::day19::part_two(&vec), Ok(848));
    // }

    // #[test]
    // fn part_two_actual() {
    //     let vec = crate::readfile::fileio::read_file(String::from("input/day19.txt"));
    //     assert_eq!(crate::day19::part_two(&vec), Ok(2136));
    // }
}
