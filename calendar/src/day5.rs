use regex::Regex;
use std::cmp;
use std::fmt::Error;

pub fn part_one(tickets: &Vec<String>) -> Result<i32, Error> {
    let mut _total = 0;
    let mut max = 0;
    for ticket in tickets.iter() {
        let row = get_row(&ticket[0..7]).unwrap();
        let col = get_col(&ticket[7..10]).unwrap();
        max = cmp::max(max, row * 8 + col);
    }

    Ok(max)
}

pub fn part_two(tickets: &Vec<String>) -> Result<i32, Error> {
    let mut seat_ids = Vec::new();
    for ticket in tickets.iter() {
        let row = get_row(&ticket[0..7]).unwrap();
        let col = get_col(&ticket[7..10]).unwrap();
        seat_ids.push(row * 8 + col);
    }

    seat_ids.sort();
    let mut ans: i32 = 0;
    for i in (1..seat_ids.len() - 1) {
        if seat_ids[i] != seat_ids[i - 1] + 1 {
            // || seat_ids[i] != seat_ids[i+1]-1
            ans = seat_ids[i];;
        }
    }
    Ok(ans - 1)
}

fn get_row(ticket: &str) -> Result<i32, Error> {
    let mut lb = 0;
    let mut ub = 127;
    let mut mp: f32 = 0.0;
    for i in ticket.chars() {
        mp = (lb as f32 + ub as f32) / 2.0;
        match i {
            'F' => {
                mp = mp.floor();
                ub = mp as i32;
            }
            'B' => {
                mp = mp.ceil();
                lb = mp as i32;
            }
            _ => {
                return Err(Error);
            }
        }
    }
    Ok(mp as i32)
}

fn get_col(ticket: &str) -> Result<i32, Error> {
    let mut lb = 0;
    let mut ub = 7;
    let mut mp: f32 = 0.0;
    for i in ticket.chars() {
        mp = (lb as f32 + ub as f32) / 2.0;
        match i {
            'L' => {
                mp = mp.floor();
                ub = mp as i32;
            }
            'R' => {
                mp = mp.ceil();
                lb = mp as i32;
            }
            _ => return Err(Error),
        }
    }
    Ok(mp as i32)
}
