use std::fmt::Error;
use std::iter::Peekable;

// https://github.com/adrianN/simple_rust_parser/blob/master/src/main.rs
// wish I could say I built this all from scratch this morning, but I heavly referenced
// this github repo, thank you adrian

#[derive(Debug, Clone)]
pub enum GrammarItem {
    Product,
    Sum,
    Number(u64),
    Paren,
}

#[derive(Debug, Clone)]
pub struct ParseNode {
    pub children: Vec<ParseNode>,
    pub entry: GrammarItem,
}

#[derive(Debug, Clone)]
pub enum LexItem {
    Paren(char),
    Op(char),
    Num(u64),
}

impl ParseNode {
    pub fn new() -> ParseNode {
        ParseNode {
            children: Vec::new(),
            entry: GrammarItem::Paren,
        }
    }
}

fn lex(input: &String) -> Result<Vec<LexItem>, String> {
    let mut result = Vec::new();
    let mut it = input.chars().peekable();
    while let Some(&c) = it.peek() {
        match c {
            '0'..='9' => {
                it.next();
                let n = get_number(c, &mut it);
                result.push(LexItem::Num(n));
            }
            '+' | '*' => {
                result.push(LexItem::Op(c));
                it.next();
            }
            '(' | ')' => {
                result.push(LexItem::Paren(c));
                it.next();
            }
            ' ' => {
                it.next();
            }
            _ => {
                return Err(format!("unexpected character {}", c));
            }
        }
    }
    Ok(result)
}

fn get_number<T: Iterator<Item = char>>(c: char, iter: &mut Peekable<T>) -> u64 {
    let mut number = c
        .to_string()
        .parse::<u64>()
        .expect("The caller should have passed a digit.");
    while let Some(Ok(digit)) = iter.peek().map(|c| c.to_string().parse::<u64>()) {
        number = number * 10 + digit;
        iter.next();
    }
    number
}

pub fn parse(input: &String) -> Result<ParseNode, String> {
    let tokens = lex(input)?;
    parse_expr(&tokens, 0).and_then(|(n, i)| {
        if i == tokens.len() {
            Ok(n)
        } else {
            Err(format!(
                "Expected end of input, found {:?} at {}",
                tokens[i], i
            ))
        }
    })
}

pub fn parse_part_one(input: &String) -> Result<ParseNode, String> {
    let tokens = lex(input)?;
    parse_part_one_expr(&tokens, 0).and_then(|(n, i)| {
        if i == tokens.len() {
            Ok(n)
        } else {
            Err(format!(
                "Expected end of input, found {:?} at {}",
                tokens[i], i
            ))
        }
    })
}

fn parse_part_one_expr(tokens: &Vec<LexItem>, pos: usize) -> Result<(ParseNode, usize), String> {
    let (node_term, next_pos) = parse_term(tokens, pos)?;
    let c = tokens.get(next_pos);
    match c {
        Some(&LexItem::Op('+')) => {
            // recurse on the part_one_expr
            let mut sum = ParseNode::new();
            sum.entry = GrammarItem::Sum;
            sum.children.push(node_term);
            let (rhs, i) = parse_part_one_expr(tokens, next_pos + 1)?;
            sum.children.push(rhs);
            Ok((sum, i))
        }
        Some(&LexItem::Op('*')) => {
            // recurse on the product
            let mut product = ParseNode::new();
            product.entry = GrammarItem::Product;
            product.children.push(node_term);
            let (rhs, i) = parse_part_one_expr(tokens, next_pos + 1)?;
            product.children.push(rhs);
            Ok((product, i))
        }
        _ => Ok((node_term, next_pos)),
    }
}

fn parse_expr(tokens: &Vec<LexItem>, pos: usize) -> Result<(ParseNode, usize), String> {
    let (node_product, next_pos) = parse_product(tokens, pos)?;
    let c = tokens.get(next_pos);
    match c {
        Some(&LexItem::Op('*')) => {
            // recurse on the product
            let mut product = ParseNode::new();
            product.entry = GrammarItem::Product;
            product.children.push(node_product);
            let (rhs, i) = parse_expr(tokens, next_pos + 1)?;
            product.children.push(rhs);
            Ok((product, i))
        }
        _ => Ok((node_product, next_pos)),
    }
}

fn parse_product(tokens: &Vec<LexItem>, pos: usize) -> Result<(ParseNode, usize), String> {
    let (node_term, next_pos) = parse_term(tokens, pos)?;
    let c = tokens.get(next_pos);
    match c {
        Some(&LexItem::Op('+')) => {
            // recurse on the expr
            let mut sum = ParseNode::new();
            sum.entry = GrammarItem::Sum;
            sum.children.push(node_term);
            let (rhs, i) = parse_product(tokens, next_pos + 1)?;
            sum.children.push(rhs);
            Ok((sum, i))
        }
        _ => Ok((node_term, next_pos)),
    }
}

fn parse_term(tokens: &Vec<LexItem>, pos: usize) -> Result<(ParseNode, usize), String> {
    let c: &LexItem = tokens.get(pos).ok_or(String::from(
        "Unexpected end of input, expected paren or number",
    ))?;
    match c {
        &LexItem::Num(n) => {
            let mut node = ParseNode::new();
            node.entry = GrammarItem::Number(n);
            Ok((node, pos + 1))
        }
        &LexItem::Paren(c) => {
            match c {
                ')' => {
                    parse_expr(tokens, pos + 1).and_then(|(node, next_pos)| {
                        if let Some(&LexItem::Paren(c2)) = tokens.get(next_pos) {
                            if c2 == '(' {
                                // okay!
                                let mut paren = ParseNode::new();
                                paren.children.push(node);
                                Ok((paren, next_pos + 1))
                            } else {
                                Err(format!("Expected ')' but found {} at {}", c2, next_pos))
                            }
                        } else {
                            Err(format!(
                                "Expected closing paren at {} but found {:?}",
                                next_pos,
                                tokens.get(next_pos)
                            ))
                        }
                    })
                }
                _ => Err(format!("Expected paren at {} but found {:?}", pos, c)),
            }
        }
        _ => Err(format!(
            "Unexpected token {:?}, expected paren or number",
            { c }
        )),
    }
}

pub fn print(tree: &ParseNode) -> String {
    match tree.entry {
        GrammarItem::Paren => {
            format!(
                "({})",
                print(tree.children.get(0).expect("parens need one child"))
            )
        }
        GrammarItem::Sum => {
            let lhs = print(tree.children.get(0).expect("sums need two children"));
            let rhs = print(tree.children.get(1).expect("sums need two children"));
            format!("{} + {}", lhs, rhs)
        }
        GrammarItem::Product => {
            let lhs = print(tree.children.get(0).expect("products need two children"));
            let rhs = print(tree.children.get(1).expect("products need two children"));
            format!("{} * {}", lhs, rhs)
        }
        GrammarItem::Number(n) => format!("{}", n),
    }
}

pub fn calculate(tree: &ParseNode) -> u64 {
    match tree.entry {
        GrammarItem::Paren => calculate(tree.children.get(0).expect("parens need one child")),
        GrammarItem::Sum => {
            let lhs = calculate(tree.children.get(0).expect("sums need two children"));
            let rhs = calculate(tree.children.get(1).expect("sums need two children"));
            lhs + rhs
        }
        GrammarItem::Product => {
            let lhs = calculate(tree.children.get(0).expect("products need two children"));
            let rhs = calculate(tree.children.get(1).expect("products need two children"));
            lhs * rhs
        }
        GrammarItem::Number(n) => n,
    }
}

// GRAMMAR
// Expr := Term + Term | Term * Term | ( Expr )
// Term := [0..9]+
pub fn part_one(program: &Vec<String>) -> Result<u64, Error> {
    let mut total = 0;
    for i in program.iter() {
        let t = i.chars().rev().collect::<String>();
        let tree = parse_part_one(&t).unwrap();
        total += calculate(&tree);
    }
    Ok(total)
}

// GRAMMAR
// Expr := Term + Term | ( Expr )
// Prod := Term * Term
// Term := [0..9]+
pub fn part_two(program: &Vec<String>) -> Result<u64, Error> {
    let mut total = 0;
    for i in program.iter() {
        let t = i.chars().rev().collect::<String>();
        let tree = parse(&t).unwrap();
        total += calculate(&tree);
    }
    Ok(total)
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_one_sample() {
        let vec = crate::readfile::fileio::read_file(String::from("input/test/day18.txt"));
        assert_eq!(crate::day18::part_one(&vec), Ok(231));
    }

    // #[test]
    // fn part_one_actual() {
    //     let vec = crate::readfile::fileio::read_file_2d(String::from("input/day18.txt"));
    //     assert_eq!(crate::day18::part_one(&vec), Ok(276));
    // }

    // #[test]
    // fn part_two_sample() {
    //     let vec = crate::readfile::fileio::read_file_2d(String::from("input/test/day18.txt"));
    //     assert_eq!(crate::day18::part_two(&vec), Ok(848));
    // }

    // #[test]
    // fn part_two_actual() {
    //     let vec = crate::readfile::fileio::read_file_2d(String::from("input/day18.txt"));
    //     assert_eq!(crate::day18::part_two(&vec), Ok(2136));
    // }
}
