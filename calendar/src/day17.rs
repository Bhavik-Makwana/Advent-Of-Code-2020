use std::fmt::Error;
const M: usize = 25;
const Z: usize = 8;
const W: usize = 8;
pub fn part_one(board: &Vec<Vec<char>>) -> Result<i32, Error> {
    let mut conway_cube: [[[char; M]; M]; M] = [[['.'; M]; M]; M];

    for col in Z..board.len() + Z {
        for row in Z..board[0].len() + Z {
            conway_cube[Z][col][row] = board[col - Z][row - Z];
        }
    }

    (0..6).for_each(|_| simulate(&mut conway_cube));

    Ok(count_active(&mut conway_cube))
}

fn count_active(conway_cube: &mut [[[char; M]; M]; M]) -> i32 {
    conway_cube
        .iter()
        .flatten()
        .flatten()
        .filter(|x| **x == '#')
        .count() as i32
}

pub fn simulate(conway_cube: &mut [[[char; M]; M]; M]) {
    let neighbours: Vec<(i32, i32, i32)> = vec![
        (1, 0, 0),
        (1, -1, 0),
        (0, -1, 0),
        (-1, -1, 0),
        (-1, 0, 0),
        (-1, 1, 0),
        (0, 1, 0),
        (1, 1, 0),
        (1, 0, 1),
        (1, -1, 1),
        (0, -1, 1),
        (-1, -1, 1),
        (-1, 0, 1),
        (-1, 1, 1),
        (0, 1, 1),
        (1, 1, 1),
        (1, 0, -1),
        (1, -1, -1),
        (0, -1, -1),
        (-1, -1, -1),
        (-1, 0, -1),
        (-1, 1, -1),
        (0, 1, -1),
        (1, 1, -1),
        (0, 0, 1),
        (0, 0, -1),
    ];
    let rows = conway_cube[0][0].len();
    let cols = conway_cube[0].len();
    let depth = conway_cube.len();

    for k in 0..depth {
        for i in 0..rows {
            for j in 0..cols {
                let mut active = 0;
                for neighbour in neighbours.iter() {
                    let r = (i as i32 + neighbour.0) as usize;
                    let c = (j as i32 + neighbour.1) as usize;
                    let d = (k as i32 + neighbour.2) as usize;

                    if r < rows && c < cols && d < depth {
                        if conway_cube[d][r][c] == '#' || conway_cube[d][r][c] == '?' {
                            active += 1;
                        }
                    }
                }

                if conway_cube[k][i][j] == '.' && active == 3 {
                    conway_cube[k][i][j] = '!';
                }
                if conway_cube[k][i][j] == '#' && (active < 2 || active > 3) {
                    conway_cube[k][i][j] = '?';
                }
            }
        }
    }

    for i in 0..rows {
        for j in 0..cols {
            for k in 0..depth {
                if conway_cube[k][i][j] == '?' {
                    conway_cube[k][i][j] = '.';
                } else if conway_cube[k][i][j] == '!' {
                    conway_cube[k][i][j] = '#';
                }
            }
        }
    }
}

pub fn part_two(board: &Vec<Vec<char>>) -> Result<i32, Error> {
    let mut conway_cube: [[[[char; M]; M]; M]; M] = [[[['.'; M]; M]; M]; M];

    for col in Z..board.len() + Z {
        for row in Z..board[0].len() + Z {
            conway_cube[W][Z][col][row] = board[col - Z][row - Z];
        }
    }

    (0..6).for_each(|_| simulate2(&mut conway_cube));

    Ok(count_active2(&mut conway_cube))
}

fn count_active2(conway_cube: &mut [[[[char; M]; M]; M]; M]) -> i32 {
    conway_cube
        .iter()
        .flatten()
        .flatten()
        .flatten()
        .filter(|x| **x == '#')
        .count() as i32
}

pub fn simulate2(conway_cube: &mut [[[[char; M]; M]; M]; M]) {
    let neighbours: Vec<(i32, i32, i32, i32)> = vec![
        (1, 0, 0, 0),
        (1, -1, 0, 0),
        (0, -1, 0, 0),
        (-1, -1, 0, 0),
        (-1, 0, 0, 0),
        (-1, 1, 0, 0),
        (0, 1, 0, 0),
        (1, 1, 0, 0),
        (1, 0, 1, 0),
        (1, -1, 1, 0),
        (0, -1, 1, 0),
        (-1, -1, 1, 0),
        (-1, 0, 1, 0),
        (-1, 1, 1, 0),
        (0, 1, 1, 0),
        (1, 1, 1, 0),
        (1, 0, -1, 0),
        (1, -1, -1, 0),
        (0, -1, -1, 0),
        (-1, -1, -1, 0),
        (-1, 0, -1, 0),
        (-1, 1, -1, 0),
        (0, 1, -1, 0),
        (1, 1, -1, 0),
        (0, 0, 1, 0),
        (0, 0, -1, 0),
        (1, 0, 0, 1),
        (1, -1, 0, 1),
        (0, -1, 0, 1),
        (-1, -1, 0, 1),
        (-1, 0, 0, 1),
        (-1, 1, 0, 1),
        (0, 1, 0, 1),
        (1, 1, 0, 1),
        (1, 0, 1, 1),
        (1, -1, 1, 1),
        (0, -1, 1, 1),
        (-1, -1, 1, 1),
        (-1, 0, 1, 1),
        (-1, 1, 1, 1),
        (0, 1, 1, 1),
        (1, 1, 1, 1),
        (1, 0, -1, 1),
        (1, -1, -1, 1),
        (0, -1, -1, 1),
        (-1, -1, -1, 1),
        (-1, 0, -1, 1),
        (-1, 1, -1, 1),
        (0, 1, -1, 1),
        (1, 1, -1, 1),
        (0, 0, 1, 1),
        (0, 0, -1, 1),
        (1, 0, 0, -1),
        (1, -1, 0, -1),
        (0, -1, 0, -1),
        (-1, -1, 0, -1),
        (-1, 0, 0, -1),
        (-1, 1, 0, -1),
        (0, 1, 0, -1),
        (1, 1, 0, -1),
        (1, 0, 1, -1),
        (1, -1, 1, -1),
        (0, -1, 1, -1),
        (-1, -1, 1, -1),
        (-1, 0, 1, -1),
        (-1, 1, 1, -1),
        (0, 1, 1, -1),
        (1, 1, 1, -1),
        (1, 0, -1, -1),
        (1, -1, -1, -1),
        (0, -1, -1, -1),
        (-1, -1, -1, -1),
        (-1, 0, -1, -1),
        (-1, 1, -1, -1),
        (0, 1, -1, -1),
        (1, 1, -1, -1),
        (0, 0, 1, -1),
        (0, 0, -1, -1),
        (0, 0, 0, -1),
        (0, 0, 0, 1),
    ];
    let rows = conway_cube[0][0][0].len();
    let cols = conway_cube[0][0].len();
    let depth = conway_cube[0].len();
    let hyper = conway_cube.len();

    for l in 0..hyper {
        for k in 0..depth {
            for i in 0..rows {
                for j in 0..cols {
                    let mut active = 0;
                    for neighbour in neighbours.iter() {
                        let r = (i as i32 + neighbour.0) as usize;
                        let c = (j as i32 + neighbour.1) as usize;
                        let d = (k as i32 + neighbour.2) as usize;
                        let h = (l as i32 + neighbour.3) as usize;

                        if r < rows && c < cols && d < depth && h < hyper {
                            if conway_cube[h][d][r][c] == '#' || conway_cube[h][d][r][c] == '?' {
                                active += 1;
                            }
                        }
                    }

                    if conway_cube[l][k][i][j] == '.' && active == 3 {
                        conway_cube[l][k][i][j] = '!';
                    }
                    if conway_cube[l][k][i][j] == '#' && (active < 2 || active > 3) {
                        conway_cube[l][k][i][j] = '?';
                    }
                }
            }
        }
    }

    for i in 0..rows {
        for j in 0..cols {
            for k in 0..depth {
                for l in 0..hyper {
                    if conway_cube[l][k][i][j] == '?' {
                        conway_cube[l][k][i][j] = '.';
                    } else if conway_cube[l][k][i][j] == '!' {
                        conway_cube[l][k][i][j] = '#';
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_one_sample() {
        let vec = crate::readfile::fileio::read_file_2d(String::from("input/test/day17.txt"));
        assert_eq!(crate::day17::part_one(&vec), Ok(112));
    }

    #[test]
    fn part_one_actual() {
        let vec = crate::readfile::fileio::read_file_2d(String::from("input/day17.txt"));
        assert_eq!(crate::day17::part_one(&vec), Ok(276));
    }

    #[test]
    fn part_two_sample() {
        let vec = crate::readfile::fileio::read_file_2d(String::from("input/test/day17.txt"));
        assert_eq!(crate::day17::part_two(&vec), Ok(848));
    }

    #[test]
    fn part_two_actual() {
        let vec = crate::readfile::fileio::read_file_2d(String::from("input/day17.txt"));
        assert_eq!(crate::day17::part_two(&vec), Ok(2136));
    }
}
