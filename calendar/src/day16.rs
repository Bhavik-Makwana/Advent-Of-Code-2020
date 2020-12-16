use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Error;

struct TicketData {
    fields: HashMap<String, HashSet<i32>>,
    valid_values: HashSet<i32>,
    your_ticket: Vec<i32>,
    nearby_tickets: Vec<Vec<i32>>,
}

pub fn part_one(input: &Vec<String>) -> Result<i32, Error> {
    let ticket_data = parse_input(&input);

    let total = ticket_data
        .nearby_tickets
        .iter()
        .flat_map(|x| x)
        .filter(|y| !ticket_data.valid_values.contains(&y))
        .sum::<i32>();
    Ok(total)
}

fn parse_input(input: &Vec<String>) -> TicketData {
    let mut fields: HashMap<String, HashSet<i32>> = HashMap::new();
    let mut valid_values: HashSet<i32> = HashSet::new();
    let mut your_ticket: Vec<i32> = Vec::new();
    let mut nearby_tickets: Vec<Vec<i32>> = Vec::new();
    let mut group = 0;
    for line in input.iter() {
        if line == "" {
            group += 1
        } else if group == 0 {
            let mut a: HashSet<i32> = HashSet::new();
            let mut x = line.split(": ");
            let key = x.next().unwrap();
            let mut y = x.next().unwrap().split(" or ");
            let mut r1 = y
                .next()
                .unwrap()
                .split("-")
                .map(|x| x.parse::<i32>().unwrap());
            for i in r1.next().unwrap()..r1.next().unwrap() + 1 {
                a.insert(i);
                valid_values.insert(i);
            }
            let mut r2 = y
                .next()
                .unwrap()
                .split("-")
                .map(|x| x.parse::<i32>().unwrap());
            for i in r2.next().unwrap()..r2.next().unwrap() + 1 {
                a.insert(i);
                valid_values.insert(i);
            }
            fields.insert(String::from(key), a);
        // fields[x[0]] = a
        // println!("{:?}", x.next());
        } else if group == 1 {
            if !line.contains("your") {
                your_ticket = line
                    .split(',')
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                // your_ticket = list(map(int,line.split(',')))
            }
        } else {
            if !line.contains("nearby") {
                nearby_tickets.push(
                    line.split(',')
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>(),
                );
                // nearby_tickets.append(list(map(int,line.split(','))))
            }
        }
    }
    TicketData {
        fields,
        valid_values,
        your_ticket,
        nearby_tickets,
    }
}

pub fn part_two(input: &Vec<String>) -> Result<i64, Error> {
    let ticket_data = parse_input(&input);
    let valid_tickets: Vec<&Vec<i32>> = ticket_data
        .nearby_tickets
        .iter()
        .filter(|x| x.iter().filter(|y| ticket_data.valid_values.contains(&y)).count() == 20)
        .collect::<Vec<&Vec<i32>>>();
    
    let mut valid_spots = HashMap::new();
    for (check, _) in ticket_data.fields.iter() {
        let mut a = Vec::new();
        for row in 0..valid_tickets[0].len() {
            let mut f = true;
            for ticket in valid_tickets.iter() {
                if !ticket_data.fields.get(check).unwrap().contains(&ticket[row]) {
                    f = false;
                    break;
                }
            }
            if f {
                a.push(row);
            }
        }
        valid_spots.insert(check, a);
    }

    let mut deps = HashMap::new();
    let mut spotted = HashSet::new();
    for i in 1..21 {
        for (key, value) in valid_spots.iter() {
            if value.len() == i {
                for j in value.iter() {
                    if !spotted.contains(&j) {
                        deps.insert(key, j);
                        spotted.insert(j);
                        break;
                    }
                }
            }
        }
    }

    let mut total: i64 = 1;
    for (key, value) in deps.iter() {
        if key.contains("departure") {
            total *= ticket_data.your_ticket[**value] as i64;
        }
    }


    Ok(total)
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_one_sample() {
        let vec = crate::readfile::fileio::read_file(String::from("input/test/day16.txt"));
        assert_eq!(crate::day16::part_one(&vec), Ok(436));
    }

    #[test]
    fn part_one_actual() {
        let vec = crate::readfile::fileio::read_file(String::from("input/day16.txt"));
        assert_eq!(crate::day16::part_one(&vec), Ok(421));
    }

    #[test]
    fn part_two_sample() {
        let vec = crate::readfile::fileio::read_file(String::from("input/test/day16.txt"));
        assert_eq!(crate::day16::part_two(&vec), Ok(175594));
    }

    #[test]
    fn part_two_actual() {
        let vec = crate::readfile::fileio::read_file(String::from("input/day16.txt"));
        assert_eq!(crate::day16::part_two(&vec), Ok(436));
    }
}
