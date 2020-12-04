// use std::collections::HashMap;
use regex::Regex;
use std::collections::HashSet;
use std::fmt::Error;

pub fn part_one(passports: &Vec<Vec<String>>) -> Result<i32, Error> {
    let mut total = 0;
    let mut map = HashSet::new();
    for passport in passports.iter() {
        for line in passport.iter() {
            for elem in line.split_whitespace() {
                let key = elem.split(":").next().unwrap();
                if key == "cid" {
                    continue;
                }
                map.insert(key);
            }
        }
        if map.len() == 7 {
            total += 1;
        }
        map = HashSet::new();
    }

    Ok(total)
}

pub fn part_two(passports: &Vec<Vec<String>>) -> Result<i32, Error> {
    let mut total = 0;
    // println!("{:?}", passports);
    let mut map = HashSet::new();
    for passport in passports.iter() {
        for line in passport.iter() {
            for elem in line.split_whitespace() {
                let mut i = elem.split(":");
                let key = i.next().unwrap();
                let value = i.next().unwrap();
                if key == "cid" {
                    continue;
                }
                if validate(key, value) {
                    map.insert(key);
                } else {
                    break;
                }
            }
        }
        if map.len() == 7 {
            total += 1;
        }
        map = HashSet::new();
    }

    Ok(total)
}

fn validate(key: &str, value: &str) -> bool {
    if key == "byr" {
        if 1920 <= value.parse::<i32>().unwrap() && value.parse::<i32>().unwrap() <= 2002 {
            return true;
        }
    } else if key == "iyr" {
        if 2010 <= value.parse::<i32>().unwrap() && value.parse::<i32>().unwrap() <= 2020 {
            return true;
        }
    } else if key == "eyr" {
        if 2020 <= value.parse::<i32>().unwrap() && value.parse::<i32>().unwrap() <= 2030 {
            return true;
        }
    } else if key == "hgt" {
        let unit = &value[value.len() - 2..value.len()];

        if unit == "cm" {
            let num = &value[0..value.len() - 2].parse::<i32>().unwrap();
            if 150 <= *num && *num <= 193 {
                return true;
            }
        } else if unit == "in" {
            let num = &value[0..value.len() - 2].parse::<i32>().unwrap();
            if 59 <= *num && *num <= 76 {
                return true;
            }
        } else {
            return false;
        }
    } else if key == "hcl" {
        let re = Regex::new(r"^#(?:[0-9a-fA-F]{6})$").unwrap();
        if re.is_match(value) {
            return true;
        }
    } else if key == "ecl" {
        let mut valid_colors = HashSet::new();
        valid_colors.insert("amb");
        valid_colors.insert("blu");
        valid_colors.insert("brn");
        valid_colors.insert("gry");
        valid_colors.insert("grn");
        valid_colors.insert("hzl");
        valid_colors.insert("oth");
        if valid_colors.contains(value) {
            return true;
        }
    } else if key == "pid" {
        if value.len() == 9 {
            return true;
        }
    }

    return false;
}
