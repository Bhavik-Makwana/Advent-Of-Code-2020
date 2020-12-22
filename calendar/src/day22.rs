use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt::Error;

pub fn part_one(input: &Vec<Vec<String>>) -> Result<i64, Error> {
    let mut batch = input.iter();
    let mut player1 = batch
        .next()
        .unwrap()
        .iter()
        .skip(1)
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<VecDeque<i64>>();
    let mut player2 = batch
        .next()
        .unwrap()
        .iter()
        .skip(1)
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<VecDeque<i64>>();

    while player1.len() > 0 && player2.len() > 0 {
        let p1_move = player1.pop_front().unwrap();
        let p2_move = player2.pop_front().unwrap();

        if p1_move > p2_move {
            player1.push_back(p1_move);
            player1.push_back(p2_move);
        } else {
            player2.push_back(p2_move);
            player2.push_back(p1_move);
        }
    }
    if player1.len() > 0 {
        Ok(player1
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, elem)| acc + (((i + 1) as i64) * elem)))
    } else {
        Ok(player2
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, elem)| acc + (((i + 1) as i64) * elem)))
    }
}

pub fn part_two(input: &Vec<Vec<String>>) -> Result<i64, Error> {
    let mut batch = input.iter();
    let mut player1 = batch
        .next()
        .unwrap()
        .iter()
        .skip(1)
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<VecDeque<i64>>();
    let mut player2 = batch
        .next()
        .unwrap()
        .iter()
        .skip(1)
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<VecDeque<i64>>();

    let (p1_win, queue) = recursive_combat(player1.clone(), player2.clone());
    Ok(queue
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, elem)| acc + (((i + 1) as i64) * elem)))
}

fn recursive_combat(
    mut player1: VecDeque<i64>,
    mut player2: VecDeque<i64>,
) -> (bool, VecDeque<i64>) {
    let mut player1_history: HashSet<String> = HashSet::new();
    let mut player2_history: HashSet<String> = HashSet::new();
    while player1.len() > 0 && player2.len() > 0 {
        let p1_deck_state = player1
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("");
        let p2_deck_state = player2
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("");

        if player1_history.contains(&p1_deck_state) || player2_history.contains(&p2_deck_state) {
            return (true, player1);
        }
        player1_history.insert(p1_deck_state);
        player2_history.insert(p2_deck_state);

        let p1_move = player1.pop_front().unwrap();
        let p2_move = player2.pop_front().unwrap();

        if player1.len() >= p1_move as usize && player2.len() >= p2_move as usize {
            let mut new_set_p1 = VecDeque::new();
            for i in 0..p1_move {
                new_set_p1.push_back(player1[i as usize]);
            }

            let mut new_set_p2 = VecDeque::new();
            for i in 0..p2_move {
                new_set_p2.push_back(player2[i as usize]);
            }
            if recursive_combat(new_set_p1.clone(), new_set_p2.clone()).0 {
                player1.push_back(p1_move);

                player1.push_back(p2_move);
            } else {
                player2.push_back(p2_move);
                player2.push_back(p1_move);
            }
        } else {
            if p1_move > p2_move {
                player1.push_back(p1_move);
                player1.push_back(p2_move);
            } else {
                player2.push_back(p2_move);
                player2.push_back(p1_move);
            }
        }
    }
    if player1.len() > 0 {
        (true, player1)
    } else {
        (false, player2)
    }
}
#[cfg(test)]
mod tests {

    #[test]
    fn part_one_sample() {
        let vec = crate::readfile::fileio::read_file_batch(String::from("input/test/day22.txt"));
        assert_eq!(crate::day22::part_one(&vec), Ok(306));
    }

    #[test]
    fn part_one_actual() {
        let vec = crate::readfile::fileio::read_file_batch(String::from("input/day22.txt"));
        assert_eq!(crate::day22::part_one(&vec), Ok(33098));
    }

    #[test]
    fn part_two_sample() {
        let vec = crate::readfile::fileio::read_file_batch(String::from("input/test/day22.txt"));
        assert_eq!(crate::day22::part_two(&vec), Ok(291));
    }

    #[test]
    fn part_two_actual() {
        let vec = crate::readfile::fileio::read_file_batch(String::from("input/day22.txt"));
        assert_eq!(crate::day22::part_two(&vec), Ok(35055));
    }
}
