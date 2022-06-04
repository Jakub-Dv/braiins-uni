use std::sync::{Arc, Mutex};
use std::{thread, time};

mod operations;
mod stdin;

use crate::operations::Operation;
use crate::stdin::{get_input_tokens, Token};

const PRINT_INTERVAL: u64 = 2;
const EVAL_INTERVAL: u64 = 1;

struct State {
    stack: Vec<i64>,
    queue: Vec<Box<dyn operations::Command>>,
}

fn main() {
    let state = Arc::new(Mutex::new(State {
        stack: Vec::new(),
        queue: Vec::new(),
    }));

    let state1 = Arc::clone(&state);
    let input_thread = thread::spawn(move || loop {
        let tokens = match get_input_tokens() {
            Some(tokens) => tokens,
            None => continue,
        };

        if let Ok(mut lock) = state1.lock() {
            for token in tokens {
                match token {
                    Token::Literal(num) => lock.queue.push(Box::new(operations::Insert::new(num))),
                    Token::Operation(op) => match op {
                        Operation::Add => lock.queue.push(Box::new(operations::Add {})),
                        Operation::Sub => lock.queue.push(Box::new(operations::Sub {})),
                        Operation::Mul => lock.queue.push(Box::new(operations::Mul {})),
                        Operation::Div => lock.queue.push(Box::new(operations::Div {})),
                    },
                    Token::Exit => lock.queue.push(Box::new(operations::Exit {})),
                };
            }
        }
        thread::sleep(time::Duration::from_secs(EVAL_INTERVAL));
    });

    let state2 = Arc::clone(&state);
    let worker_thread = thread::spawn(move || loop {
        if let Ok(mut lock) = state2.lock() {
            while let Some(token) = &lock.queue.pop() {
                token.execute(&mut lock.stack)
            }
        }
        thread::sleep(time::Duration::from_secs(EVAL_INTERVAL));
    });

    let state3 = Arc::clone(&state);
    let print_thread = thread::spawn(move || loop {
        if let Ok(mut lock) = state3.lock() {
            while let Some(val) = &lock.stack.pop() {
                println!("{}", val);
            }
        }
        thread::sleep(time::Duration::from_secs(PRINT_INTERVAL));
    });

    input_thread.join().expect("");
    worker_thread.join().expect("");
    print_thread.join().expect("");
}
