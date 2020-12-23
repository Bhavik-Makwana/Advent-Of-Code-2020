use itertools::Itertools;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fmt::Error;
use std::fs;
use std::iter::Iterator;

pub fn part_one(input: &String) -> Result<String, Error> {
    let mut cups = input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i8)
        .collect::<VecDeque<i8>>();
    for i in 0..100 {
        let current_cup = cups.pop_front().unwrap();
        let next = cups.drain(0..3).collect::<Vec<_>>();
        cups.push_back(current_cup);
        let destination = cups
            .iter()
            .map(|&x| {
                if x < current_cup {
                    current_cup - x
                } else {
                    current_cup + 9 - x
                }
            })
            .enumerate()
            .sorted_by(|(_, a), (_, b)| Ord::cmp(a, b))
            .map(|(i, _)| i)
            .next()
            .unwrap()
            + 1;
        next.iter().rev().for_each(|&x| cups.insert(destination, x));
    }

    println!("result: {:?}", cups);

    Ok(String::from("ok"))
}

#[derive(Clone, Debug)]
struct Node {
    prev: usize,
    next: usize,
}

fn round(cups: &mut Vec<Node>, current_cup: &mut usize) {
    let adj1 = cups[*current_cup].next;
    let adj2 = cups[adj1].next;
    let adj3 = cups[adj2].next;

    let mut dest_cup = if *current_cup == 1 {
        cups.len() - 1
    } else {
        *current_cup - 1
    };

    while vec![adj1, adj2, adj3].contains(&dest_cup) {
        dest_cup = if dest_cup == 1 {
            cups.len() - 1
        } else {
            dest_cup - 1
        };
    }
    
    cups[*current_cup].next = cups[adj3].next;
    let post_dest = cups[dest_cup].next;
    cups[dest_cup].next = adj1;
    cups[adj1] = Node {
        prev: dest_cup,
        next: adj2,
    };
    cups[adj2] = Node {
        prev: adj1,
        next: adj3,
    };
    cups[adj3] = Node {
        prev: adj2,
        next: post_dest,
    };
    cups[post_dest].prev = adj3;
    *current_cup = cups[*current_cup].next;
}

fn to_linked(cups: &[usize]) -> Vec<Node> {
    let mut linked_list = vec![Node { next: 0, prev: 0 }; cups.len() + 1];
    for window in cups.windows(3) {
        linked_list[window[1] as usize] = Node {
            prev: window[0],
            next: window[2],
        };
    }
    // start
    linked_list[cups[0] as usize] = Node {
        prev: cups[cups.len() - 1],
        next: cups[1],
    };
    // end
    linked_list[cups[cups.len() - 1] as usize] = Node {
        prev: cups[cups.len() - 2],
        next: cups[0],
    };
    linked_list
}

pub fn part_two(input: &String) -> Result<usize, Error> {
    let mut cups: Vec<usize> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    cups.extend(cups.iter().copied().max().unwrap() + 1..=1000000);
    let mut current_cup = cups[0];
    let mut cups = to_linked(&cups);
    for _ in 0..10000000 {
        round(&mut cups, &mut current_cup);
    }
    let l1 = cups[1].next;
    let l2 = cups[l1].next;
    Ok(l1 * l2)
}

#[cfg(test)]
mod tests {

    // #[test]
    // fn part_one_sample() {
    //     // let vec = crate::readfile::fileio::read_file_batch(String::from("input/test/day23.txt"));
    //     let vec: String = String::from("389125467");
    //     assert_eq!(crate::day23::part_one(&vec), Ok(String::from("67384529")));
    // }

    // #[test]
    // fn part_one_actual() {
    //     let vec = crate::readfile::fileio::read_file_batch(String::from("input/day23.txt"));
    //     assert_eq!(crate::day23::part_one(&vec), Ok(33098));
    // }

    #[test]
    fn part_two_sample() {
        let vec: String = String::from("389125467");
        assert_eq!(crate::day23::part_two(&vec), Ok(149245887792));
    }

    // #[test]
    // fn part_two_actual() {
    //     let vec = crate::readfile::fileio::read_file_batch(String::from("input/day23.txt"));
    //     assert_eq!(crate::day23::part_two(&vec), Ok(35055));
    // }
}
