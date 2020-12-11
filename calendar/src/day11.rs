use std::fmt::Error;

pub fn part_one(input: &Vec<Vec<char>>) -> Result<i32, Error> {
    let mut board = input.clone();
    let mut count = count_seats(&mut board);

    game_of_seats(&mut board);
    while count != count_seats(&mut board) {
        count = count_seats(&mut board);
        game_of_seats(&mut board);
    }
    Ok(count)
}

fn count_seats(board: &mut Vec<Vec<char>>) -> i32{
    let x = board.iter().fold(0, |c, x| c+x.iter().filter(|s| **s == '#').count());
    x as i32
}

pub fn game_of_seats(board: &mut Vec<Vec<char>>) {
    let neighbours: Vec<(i32, i32)> = vec![(1,0), (1,-1), (0,-1), (-1,-1), (-1,0), (-1,1), (0,1), (1,1)];
    let rows = board.len();
    let cols = board[0].len();

    for i in 0..rows {
        for j in 0..cols {
            if board[i][j] == '.' {
                continue;
            }
            let mut occupied = 0;
            for neighbour in neighbours.iter() {
                let r = (i as i32 + neighbour.0) as usize;
                let c = (j as i32 + neighbour.1) as usize;

                if r < rows  && c < cols  {
                    if board[r][c] == '#' || board[r][c] == '?' {
                        occupied += 1;
                    }
                }
            }
            
            if board[i][j] == 'L' && occupied == 0 {
                board[i][j] = '!';
            }
            if board[i][j] == '#' && occupied >= 4 {
                board[i][j] = '?';
            }
        }
    }

    for i in 0..rows {
        for j in 0..cols {
            if board[i][j] == '?' {
                board[i][j] = 'L';
            }
            else if board[i][j] == '!' {
                board[i][j] = '#';
            }
        }
    }
}
pub fn part_two(input: &Vec<Vec<char>>) -> Result<i32, Error> {
    let mut board = input.clone();
    game_of_seats2(&mut board);
    game_of_seats2(&mut board);
    let mut count = count_seats(&mut board);
    game_of_seats2(&mut board);
    while count != count_seats(&mut board) {
        count = count_seats(&mut board);
        // println!("{}", count);
        game_of_seats2(&mut board);
    }
    Ok(count)

}

pub fn game_of_seats2(board: &mut Vec<Vec<char>>) {
    let neighbours: Vec<Vec<i32>> = vec![vec![1,0], vec![1,-1], vec![0,-1],
    vec![-1,-1], vec![-1,0], vec![-1,1], vec![0,1], vec![1,1]];
    let rows = board.len();
    let cols = board[0].len();

    for i in 0..rows {
        for j in 0..cols {
            if board[i][j] == '.' {
                continue;
            }
            let mut occupied = 0;
            for neighbour in neighbours.iter() {
                let mut r = (i as i32 + neighbour[0]) as usize;
                let mut c = (j as i32 + neighbour[1]) as usize;

                while r < rows && c < cols  {
                    if board[r][c] == '#' || board[r][c] == '?' {
                        occupied += 1;
                        break;
                    }
                    else if board[r][c] == 'L' || board[r][c] == '!' {
                        break;
                    }
                    else {
                        r = (r as i32 + neighbour[0]) as usize;
                        c = (c as i32 + neighbour[1]) as usize;
                    }
                }
            }
            
            if board[i][j] == 'L' && occupied == 0 {
                board[i][j] = '!';
            }
            if board[i][j] == '#' && occupied >= 5 {
                board[i][j] = '?';
            }
        }
    }
    for i in 0..rows {
        for j in 0..cols {
            if board[i][j] == '?' {
                board[i][j] = 'L';
            }
            else if board[i][j] == '!' {
                board[i][j] = '#';
            }
        }
    }
}



#[cfg(test)]
mod tests {

    #[test]
    fn part_one_sample() {
        let vec = crate::readfile::fileio::read_file_2d(String::from("input/test/day11.txt"));
        assert_eq!(crate::day11::part_one(&vec), Ok(37));
    }

    #[test]
    fn part_one_actual() {
        let vec = crate::readfile::fileio::read_file_2d(String::from("input/day11.txt"));
        assert_eq!(crate::day11::part_one(&vec), Ok(2261));
    }

    #[test]
    fn part_two_sample() {
        let vec = crate::readfile::fileio::read_file_2d(String::from("input/test/day11.txt"));
        assert_eq!(crate::day11::part_two(&vec), Ok(26));
    }

    #[test]
    fn part_two_actual() {
        let vec = crate::readfile::fileio::read_file_2d(String::from("input/day11.txt"));
        // let start_time = Utc::now().time();
        assert_eq!(crate::day11::part_two(&vec), Ok(2039));
        // let end_time = Utc::now().time();
        // let diff = end_time - start_time;
        // println!("time: {}", diff);
    }
}
