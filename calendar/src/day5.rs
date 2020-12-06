use std::fmt::Error;

pub fn part_one(tickets: &Vec<String>) -> Option<i32> {
    tickets.iter().map(|ticket| get_seat_id(ticket)).max()
}

pub fn part_two(tickets: &Vec<String>) -> Result<i32, Error> {
    let mut ans: i32 = 0;
    let mut seat_ids: Vec<i32> = tickets.iter().map(|ticket| get_seat_id(ticket)).collect();
    seat_ids.sort();
    // let x: Vec<_> = (1..seat_ids.len()-1).filter(|i| seat_ids[*i as usize] != seat_ids[(*i as usize)-1] +1).collect();
    // println!("{:?}", x);
    for i in 1..seat_ids.len()-1 {
        if seat_ids[i] != seat_ids[i - 1] + 1 {
            ans = seat_ids[i];
        }
    }
    Ok(ans - 1)
}

fn _get_seat_id_functional(ticket: &str) -> i32 {
    let row: String = String::from(&ticket[0..7]).chars()
    .map(|x| if x == 'B' { '1'} else{ x})
    .map(|x| if x == 'F' { '0'} else{ x})
    .collect();
    let col: String = String::from(&ticket[7..10]).chars()
    .map(|x| if x == 'R' { '1'} else{ x})
    .map(|x| if x == 'L' { '0'} else{ x})
    .collect();
    
    let row_int = i32::from_str_radix(&row[..], 2).unwrap();
    let col_int = i32::from_str_radix(&col[..], 2).unwrap();
    row_int * 8 + col_int
}

fn get_seat_id(ticket: &str) -> i32 {
    let row = &ticket[0..7].replace("B", "1").replace("F", "0");
    let col = &ticket[7..10].replace("L", "0").replace("R", "1");
    let row_int = i32::from_str_radix(&row, 2).unwrap();
    let col_int = i32::from_str_radix(&col, 2).unwrap();
    row_int * 8 + col_int
}