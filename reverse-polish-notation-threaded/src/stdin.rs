use std::io;

use std::fmt::{Display, Formatter};

use crate::operations::Operation;

pub enum Token {
    Literal(i64),
    Operation(Operation),
    Exit,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Token::Literal(num) => num.to_string(),
                Token::Operation(op) => op.to_string(),
                Token::Exit => "q".to_string(),
            }
        )
    }
}

fn read_line() -> Result<String, io::Error> {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line)?;
    Ok(line)
}

pub fn get_input_tokens() -> Option<Vec<Token>> {
    let string_tokens = match read_line() {
        Ok(orig_line) => orig_line,
        Err(_) => return None,
    };
    let string_tokens = match string_tokens.strip_suffix('\n') {
        Some(stripped_line) => stripped_line,
        None => string_tokens.as_str(),
    };
    let mut tokens = string_tokens
        .split(' ')
        .filter(|token| !token.is_empty())
        .map(|token| match token.parse::<i64>() {
            Ok(num) => Token::Literal(num),
            Err(_) => match token {
                "+" => Token::Operation(Operation::Add),
                "-" => Token::Operation(Operation::Sub),
                "*" => Token::Operation(Operation::Mul),
                "/" => Token::Operation(Operation::Div),
                "q" | "Q" => Token::Exit,
                other => panic!("Got unexpected character: {}", other),
            },
        })
        .collect::<Vec<Token>>();

    tokens.reverse();
    Some(tokens)
}
