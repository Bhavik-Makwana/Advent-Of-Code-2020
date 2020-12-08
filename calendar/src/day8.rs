use std::fmt::Error;

#[derive(Debug, Clone)]
enum Instruction {
    ACC(i32),
    JMP(i32),
    NOP(i32),
}

pub fn part_one(raw_program: &Vec<String>) -> Result<i32, Error> {
    let mut program = parse_program(&raw_program);
    let res = terminates(&mut program, 0);
    Ok(res.1)
}

pub fn part_two(raw_program: &Vec<String>) -> Result<i32, Error> {
    let mut program = parse_program(&raw_program);
    let mut i = 0;
    let mut x: (i32, i32) = (0, 0);
    let n = program.len() as i32;
    while x.0 != n - 1 {
        let tmp = program[i].0.clone();
        match program[i].0 {
            Instruction::JMP(operand) => {
                program[i].0 = Instruction::NOP(operand);
                x = terminates(&mut program, 0);
                if x.0 >= n - 1 {
                    break;
                }
                program[i].0 = tmp;
            }
            Instruction::NOP(operand) => {
                program[i].0 = Instruction::JMP(operand);
                x = terminates(&mut program, 0);
                if x.0 >= n - 1 {
                    break;
                }
                program[i].0 = tmp;
            }
            Instruction::ACC(_) => (),
        }

        i += 1;
        reset_prog(&mut program);
    }

    Ok(x.1)
}

fn reset_prog(program: &mut Vec<(Instruction, bool)>) {
    for line in program {
        line.1 = false;
    }
}

fn parse_program(raw_program: &Vec<String>) -> Vec<(Instruction, bool)> {
    let mut program: Vec<(Instruction, bool)> = Vec::new();
    for line in raw_program.iter() {
        let mut x = line.split_whitespace();
        let opcode = x.next();
        let operand = x.next().unwrap().parse::<i32>().unwrap();
        match opcode {
            Some("jmp") => program.push((Instruction::JMP(operand), false)),
            Some("nop") => program.push((Instruction::NOP(operand), false)),
            Some("acc") => program.push((Instruction::ACC(operand), false)),
            None => println!("Huh"),
            _ => panic!("Invalid program"),
        };
    }
    program
}

fn terminates(program: &mut Vec<(Instruction, bool)>, mut pointer: usize) -> (i32, i32) {
    let mut acc = 0;
    loop {
        if pointer == program.len() || program[pointer].1 == true {
            return (pointer as i32, acc);
        }

        match program[pointer].0 {
            Instruction::NOP(_) => {
                program[pointer].1 = true;
                pointer += 1;
                continue;
            }
            Instruction::ACC(operand) => {
                program[pointer].1 = true;
                acc += operand;
                pointer += 1;
            }
            Instruction::JMP(operand) => {
                program[pointer].1 = true;
                pointer = (pointer as i32 + operand) as usize;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one_sample() {
        let vec = crate::readfile::fileio::read_file(String::from("input/test/day8.txt"));
        assert_eq!(crate::day8::part_one(&vec), Ok(5));
    }

    #[test]
    fn part_one_actual() {
        let vec = crate::readfile::fileio::read_file(String::from("input/day8.txt"));
        assert_eq!(crate::day8::part_one(&vec), Ok(1766));
    }

    #[test]
    fn part_two_sample() {
        let vec = crate::readfile::fileio::read_file(String::from("input/test/day8.txt"));
        assert_eq!(crate::day8::part_two(&vec), Ok(8));
    }

    #[test]
    fn part_two_actual() {
        let vec = crate::readfile::fileio::read_file(String::from("input/day8.txt"));
        assert_eq!(crate::day8::part_two(&vec), Ok(1639));
    }
}
