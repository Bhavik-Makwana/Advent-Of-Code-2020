use std::collections::HashMap;
use std::fmt::Error;

pub fn part_one(input: &Vec<String>) -> Result<i64, Error> {
    let mut codes: Vec<Vec<i64>> = Vec::new();
    let mut mask = String::from("");
    let mut mem = HashMap::new();
    for line in input.iter() {
        if line.contains("mask") {
            let mut iter = line.split(" ");
            iter.next();
            iter.next();
            mask = String::from(iter.next().unwrap());
            codes = Vec::new();
        } else {
            let mut x = line.replace("mem[", "");
            x = x.replace("]", "");
            x = x.replace("=", "");
            x = x.replace("  ", " ");
            let mut iter = x.split(" ");
            let addr = iter.next().unwrap().parse::<i64>().unwrap();
            let instruction = iter.next().unwrap().parse::<i64>().unwrap();
            codes.push(vec![addr, instruction]);
            update_memory(&mask, &codes, &mut mem);
        }
    }
    let mut total: i64 = 0;
    for (key, value) in mem.iter() {
        println!("{} {}", key, value);
        total += value;
    }
    Ok(total)
}

fn update_memory(mask: &String, codes: &Vec<Vec<i64>>, mem: &mut HashMap<i64, i64>) {
    for code in codes.iter() {
        let address = code[0];
        let mut inst: Vec<char> = format!("{:036b}", code[1]).chars().collect();
        for (i, m) in mask.chars().enumerate() {
            if m == '0' || m == '1' {
                inst[i] = m;
            }
        }
        let inst_str: String = inst.iter().collect();
        mem.insert(address, i64::from_str_radix(&inst_str, 2).unwrap());
    }
}

pub fn part_two(input: &Vec<String>) -> Result<i64, Error> {
    let mut codes: Vec<Vec<i64>> = Vec::new();
    let mut mask = String::from("");
    let mut mem = HashMap::new();
    for line in input.iter() {
        if line.contains("mask") {
            let mut iter = line.split(" ");
            iter.next();
            iter.next();
            mask = String::from(iter.next().unwrap());
            codes = Vec::new();
        } else {
            let mut x = line.replace("mem[", "");
            x = x.replace("]", "");
            x = x.replace("=", "");
            x = x.replace("  ", " ");
            let mut iter = x.split(" ");
            let addr = iter.next().unwrap().parse::<i64>().unwrap();
            let instruction = iter.next().unwrap().parse::<i64>().unwrap();
            codes.push(vec![addr, instruction]);
            update_memory2(&mask, &codes, &mut mem);
        }
    }
    let mut total: i64 = 0;
    for (key, value) in mem.iter() {
        total += value;
    }
    Ok(total)
}

fn update_memory2(mask: &String, codes: &Vec<Vec<i64>>, mem: &mut HashMap<i64, i64>) {
    for code in codes.iter() {
        let mut address: Vec<char> = format!("{:036b}", code[0]).chars().collect();
        let inst = code[1];
        let mut xs = 0;
        for (i, m) in mask.chars().enumerate() {
            match m {
                'X' => {
                    xs += 1;
                    address[i] = m;
                }
                '1' => address[i] = m,
                _ => (),
            }
        }
        let bin_str: Vec<char> = vec![];
        generate_binary(xs, bin_str, &mut address, inst, mem);
        
    }
}

fn generate_binary(
    length: usize,
    string:  Vec<char>,
    address: &mut Vec<char>,
    value: i64,
    mem: &mut HashMap<i64, i64>,
) {
    if (length > 0) {
        let mut string_clone = string.clone();
        string_clone.push('0');
        generate_binary(length - 1, string_clone, address, value, mem);
        let mut string_clone = string.clone();
        string_clone.push('1');
        generate_binary(length - 1, string_clone, address, value, mem);
    } else {
        let mut j = 0;
        let mut addr = address.clone();
        for i in 0..addr.len() {
            if addr[i] == 'X' {
                addr[i] = string[j];
                j += 1;
            }
        }
        let addr_str: String = addr.iter().collect();
        mem.insert(i64::from_str_radix(&addr_str, 2).unwrap(), value);
        
    }
}

#[cfg(test)]
mod tests {

    // #[test]
    // fn part_one_sample() {
    //     let vec = crate::readfile::fileio::read_file(String::from("input/test/day14.txt"));
    //     assert_eq!(crate::day14::part_one(&vec), Ok(165));
    // }

    #[test]
    fn part_one_actual() {
        let vec = crate::readfile::fileio::read_file(String::from("input/day14.txt"));
        assert_eq!(crate::day14::part_one(&vec), Ok(16003257187056));
    }

    #[test]
    fn part_two_sample() {
        let vec = crate::readfile::fileio::read_file(String::from("input/test/day14.txt"));
        assert_eq!(crate::day14::part_two(&vec), Ok(208));
    }

    #[test]
    fn part_two_actual() {
        let vec = crate::readfile::fileio::read_file(String::from("input/day14.txt"));
        assert_eq!(crate::day14::part_two(&vec), Ok(3219837697833));
    }
}
