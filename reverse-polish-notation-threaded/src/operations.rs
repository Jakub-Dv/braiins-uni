use std::fmt::{Display, Formatter};

pub trait Command: Sync + Send {
    fn execute(&self, stack: &mut Vec<i64>);
}

pub enum Operation {
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
                Operation::Add => "Add",
                Operation::Sub => "Sub",
                Operation::Div => "Div",
                Operation::Mul => "Mul",
            }
        )
    }
}

pub struct Insert {
    num: i64,
}
impl Insert {
    pub fn new(num: i64) -> Self {
        Self { num }
    }
}

pub struct Add {}
pub struct Sub {}
pub struct Mul {}
pub struct Div {}
pub struct Exit {}

impl Command for Insert {
    fn execute(&self, stack: &mut Vec<i64>) {
        stack.push(self.num);
    }
}

type Operands = (i64, i64);
fn get_operand_from_stack(stack: &mut Vec<i64>) -> Option<Operands> {
    let right = match &stack.pop() {
        Some(num) => *num,
        None => return None,
    };
    let left = match &stack.pop() {
        Some(num) => *num,
        None => return None,
    };
    Some((left, right))
}

impl Command for Add {
    fn execute(&self, stack: &mut Vec<i64>) {
        let (left, right) = match get_operand_from_stack(stack) {
            Some(op) => (op.0, op.1),
            None => return,
        };
        stack.push(left + right);
    }
}

impl Command for Sub {
    fn execute(&self, stack: &mut Vec<i64>) {
        let (left, right) = match get_operand_from_stack(stack) {
            Some(op) => (op.0, op.1),
            None => return,
        };
        stack.push(left - right);
    }
}

impl Command for Mul {
    fn execute(&self, stack: &mut Vec<i64>) {
        let (left, right) = match get_operand_from_stack(stack) {
            Some(op) => (op.0, op.1),
            None => return,
        };
        stack.push(left * right);
    }
}

impl Command for Div {
    fn execute(&self, stack: &mut Vec<i64>) {
        let (left, right) = match get_operand_from_stack(stack) {
            Some(op) => (op.0, op.1),
            None => return,
        };
        stack.push(left / right);
    }
}

impl Command for Exit {
    fn execute(&self, _: &mut Vec<i64>) {
        std::process::exit(0);
    }
}
