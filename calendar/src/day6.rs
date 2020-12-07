// use std::collections::HashMap;
use regex::Regex;
use std::collections::HashSet;
use std::fmt::Error;

pub fn part_one(forms: &Vec<Vec<String>>) -> Result<i32, Error> {
    let mut total: i32 = 0;
    
    for form in forms.iter() {
        total += form_set(form).len() as i32;
    }
    
    Ok(total)
}

pub fn part_two(forms: &Vec<Vec<String>>) -> Result<i32, Error> {
    let mut total: i32 = 0;
    

    for form in forms.iter() {
        let mut sets = Vec::<HashSet<char>>::new();
        for (i, line) in form.iter().enumerate() {
            let mut s = HashSet::<char>::new();
            for elem in line.chars() {
                    s.insert(elem);
            }
            sets.push(s);

            
        } 
        
        let intersection = sets
        .iter()
        .skip(1)
        .fold(sets[0].clone(), |a, b| {
            a.intersection(b).cloned().collect()
        });

        total += intersection.len() as i32;
 
    }
    Ok(total)
}

fn form_set(form: &Vec<String>) -> HashSet<char> {
    let mut s = HashSet::new();
    for line in form.iter() {
        for elem in line.chars() {
            s.insert(elem);
        }
    }
    s
}