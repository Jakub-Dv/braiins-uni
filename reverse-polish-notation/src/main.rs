use std::fmt::{Display, Formatter};
use std::io;
use std::sync::Arc;

enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operation::Add => "+",
                Operation::Sub => "-",
                Operation::Mul => "*",
                Operation::Div => "/",
            }
        )
    }
}

enum Token {
    Literal(i64),
    Operation(Operation),
    Compute(Arc<dyn Command>),
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Token::Literal(num) => num.to_string(),
                Token::Operation(op) => format!("{}", op),
                Token::Compute(cmp) => format!("Compute<{}>", cmp),
            }
        )
    }
}

trait Command: Display {
    fn execute(&self) -> i64;
}

struct Expression {
    left: Token,
    right: Token,
    operation: Operation,
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}<left: {}, right: {}>",
            self.operation, self.left, self.right
        )
    }
}

impl Command for Expression {
    fn execute(&self) -> i64 {
        let left = match &self.left {
            Token::Literal(num) => *num,
            Token::Compute(cmd) => cmd.execute(),
            _ => panic!("Not expecting Operation here"),
        };
        let right = match &self.right {
            Token::Literal(num) => *num,
            Token::Compute(cmd) => cmd.execute(),
            _ => panic!("Not expecting Operation here"),
        };
        match self.operation {
            Operation::Add => left + right,
            Operation::Sub => left - right,
            Operation::Mul => left * right,
            Operation::Div => left / right,
        }
    }
}

fn read_line() -> Result<String, io::Error> {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line)?;
    Ok(line)
}

fn get_input_tokens() -> Option<Vec<Token>> {
    let string_tokens = match read_line() {
        Ok(orig_line) => orig_line,
        Err(_) => return None,
    };
    let string_tokens = match string_tokens.strip_suffix("\n") {
        Some(stripped_line) => stripped_line,
        None => string_tokens.as_str(),
    };
    Some(
        string_tokens
            .split(" ")
            .filter(|token| !token.is_empty())
            .map(|token| match token.parse::<i64>() {
                Ok(num) => Token::Literal(num),
                Err(_) => match token {
                    "+" => Token::Operation(Operation::Add),
                    "-" => Token::Operation(Operation::Sub),
                    "*" => Token::Operation(Operation::Mul),
                    "/" => Token::Operation(Operation::Div),
                    "q" | "Q" => std::process::exit(0),
                    other => panic!("Got unexpected character: {}", other),
                },
            })
            .collect::<Vec<Token>>(),
    )
}

fn build_operation_tree(mut tokens: Vec<Token>) -> Result<Arc<dyn Command>, ()> {
    if tokens.len() == 1 {
        return match tokens.pop().unwrap() {
            Token::Literal(_) => Err(()),
            Token::Operation(_) => Err(()),
            Token::Compute(cmp) => Ok(cmp),
        };
    }
    tokens.reverse();
    let mut new_tokens: Vec<Token> = Vec::new();
    let mut include_op = false;
    while let Some(token) = tokens.pop() {
        match token {
            Token::Operation(operation) => {
                include_op = true;
                let right = match new_tokens.pop() {
                    Some(token) => token,
                    None => return Err(()),
                };
                let left = match new_tokens.pop() {
                    Some(token) => token,
                    None => return Err(()),
                };
                &new_tokens.push(Token::Compute(Arc::new(Expression {
                    left,
                    right,
                    operation,
                })))
            }
            token => &new_tokens.push(token),
        };
    }
    if include_op {
        build_operation_tree(new_tokens)
    } else {
        Err(())
    }
}

fn main() {
    loop {
        let tokens = match get_input_tokens() {
            Some(tokens) => tokens,
            None => return,
        };

        let result = match build_operation_tree(tokens) {
            Ok(cmd) => cmd.execute(),
            Err(_) => std::process::exit(1),
        };
        println!("{}", result);
    }
}
