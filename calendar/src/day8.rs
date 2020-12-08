use std::fmt::Error;

pub fn part_one(raw_program: &Vec<String>) -> Result<i32, Error> {
    let mut program = parse_program(&raw_program);
    let res = terminates(&mut program, 0);
    Ok(res.1)
}

pub fn part_two(raw_program: &Vec<String>) -> Result<i32, Error> {
    let mut program = parse_program(&raw_program);
    let mut i = 0;
    let mut x: (i32, i32) = (0, 0);
    let n = (program.len() as i32);
    while x.0 != n - 1  {
        let tmp = program[i].0.clone();
        if program[i].0 == "jmp".to_string() {
            program[i].0 = "nop".to_string();
            x = terminates(&mut program, 0);
            if x.0 >= n - 1 {
                break;
            }
            program[i].0 = tmp;
        } else if program[i].0 == "nop".to_string() {
            program[i].0 = "jmp".to_string();
            x = terminates(&mut program, 0);
            if x.0 >= n - 1 {
                break;
            }
            program[i].0 = tmp;
        }
        i += 1;
        reset_prog(&mut program);
    }

    Ok(x.1)
}

fn reset_prog(program: &mut Vec<(String, i32, bool)>) {
    for line in program {
        line.2 = false;
    }
}

fn parse_program(raw_program: &Vec<String>) -> Vec<(String, i32, bool)> {
    let mut program: Vec<(String, i32, bool)> = Vec::new();
    for line in raw_program.iter() {
        let mut x = line.split_whitespace();
        let opcode = x.next().unwrap().to_string();
        let operand: i32 = x.next().unwrap().parse::<i32>().unwrap();
        program.push((opcode, operand, false));
    }
    program
}

fn terminates(program: &mut Vec<(String, i32, bool)>, mut pointer: usize) -> (i32, i32) {
    let mut total = 0;
    while true {
        if pointer == program.len() {
            return (pointer as i32, total);
        }
        if program[pointer].2 == true {
            return (pointer as i32, total);
        }
        if program[pointer].0 == "nop" {
            program[pointer].2 = true;
            pointer += 1;
            continue;
        } else if program[pointer].0 == "acc" {
            program[pointer].2 = true;
            total += program[pointer].1;
            pointer += 1;
        } else if program[pointer].0 == "jmp" {
            program[pointer].2 = true;
            pointer = (pointer as i32 + program[pointer].1) as usize;
        }
    }
    (-1, -1)
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one_sample() {
        let vec = crate::readfile::fileio::read_file(String::from("input/test/day8.txt"));
        assert_eq!(crate::day8::part_one(&vec), Ok(5));
    }

    #[test]
    fn part_two_sample() {
        let vec = crate::readfile::fileio::read_file(String::from("input/test/day8.txt"));
        assert_eq!(crate::day8::part_two(&vec), Ok(8));
    }
}
