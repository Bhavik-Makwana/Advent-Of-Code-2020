use std::fmt::Error;

pub fn part_one(input: &Vec<String>) -> Result<i32, Error> {
    let departure_time = input[0].parse::<i32>().unwrap();
    let timetable = input[1].split(',').collect::<Vec<&str>>();
    let mut min_times = Vec::new();
    for bus in timetable.iter() {
        match bus {
            &"x" => min_times.push(i32::MAX),
            _ => {
                let mut checker = 0;
                while checker < departure_time {
                    checker += bus.parse::<i32>().unwrap();
                }
                min_times.push(checker);
            }
        }
    }
    let (index, max) =
        min_times
            .iter()
            .enumerate()
            .fold((0, min_times[0]), |(index, max), (i, elem)| {
                if elem < &max {
                    (i, *elem)
                } else {
                    (index, max)
                }
            });

    Ok((max - departure_time) * timetable[index].parse::<i32>().unwrap())
}

// https://www.dave4math.com/mathematics/chinese-remainder-theorem/
// https://brilliant.org/wiki/chinese-remainder-theorem/
pub fn part_two(input: &Vec<String>) -> Result<i128, Error> {
    let timetable = input[1].split(',').collect::<Vec<&str>>();
    let M: i128 = timetable
        .iter()
        .filter(|x| *x != &"x")
        .map(|x| x.parse::<i128>().unwrap())
        .product();

    let ans = timetable
        .iter()
        .enumerate()
        .filter(|(_, x)| *x != &"x")
        .fold(0, |acc, (i, bus)| acc + crt(bus, i, M));

    Ok(ans.rem_euclid(M))
}

fn crt(bus: &str, i: usize, M: i128) -> i128 {
    let bus_num = bus.parse::<i128>().unwrap();
    let a = bus_num - i as i128;
    let y = M / bus_num;
    let z = gcd_extended(y, bus_num).1;
    a*y*z
}

// ref GeeksForGeeks
fn gcd_extended(a: i128, b: i128) -> (i128, i128, i128) {
    // Base Case
    if a == 0 {
        return (b, 0, 1);
    }
    let (gcd, x1, y1) = gcd_extended(b % a, a);

    // Update x and y using results of recursive call
    let x = y1 - (b / a) * x1;
    let y = x1;

    (gcd, x, y)
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_one_sample() {
        let vec = crate::readfile::fileio::read_file(String::from("input/test/day13.txt"));
        assert_eq!(crate::day13::part_one(&vec), Ok(295));
    }

    #[test]
    fn part_one_actual() {
        let vec = crate::readfile::fileio::read_file(String::from("input/day13.txt"));
        assert_eq!(crate::day13::part_one(&vec), Ok(207));
    }

    #[test]
    fn part_two_sample() {
        let vec = crate::readfile::fileio::read_file(String::from("input/test/day13.txt"));
        assert_eq!(crate::day13::part_two(&vec), Ok(1068781));
    }

    #[test]
    fn part_two_actual() {
        let vec = crate::readfile::fileio::read_file(String::from("input/day13.txt"));
        // let start_time = Utc::now().time();
        assert_eq!(crate::day13::part_two(&vec), Ok(530015546283687));
        // let end_time = Utc::now().time();
        // let diff = end_time - start_time;
        // println!("time: {}", diff);
    }
}
