pub mod calculator_service {
    tonic::include_proto!("calculator_service");
}

use calculator_service::calculator_service_client::CalculatorServiceClient;
use calculator_service::CalcInput;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::io::stdin;
use std::process::exit;

#[derive(Debug)]
struct ResultError {
    operation: Operation,
}

impl Display for ResultError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.operation {
            Operation::Add(a, b) => write!(f, "Failed to compute: a={}, b={}, op=+", a, b),
            Operation::Sub(a, b) => write!(f, "Failed to compute: a={}, b={}, op=-", a, b),
            Operation::Mul(a, b) => write!(f, "Failed to compute: a={}, b={}, op=*", a, b),
            Operation::Div(a, b) => write!(f, "Failed to compute: a={}, b={}, op=/", a, b),
        }
    }
}
impl Error for ResultError {}

#[derive(Debug)]
enum Operation {
    Add(i64, i64),
    Sub(i64, i64),
    Mul(i64, i64),
    Div(i64, i64),
}

fn parse_input(input: String) -> Result<Operation, Box<dyn Error>> {
    let mut tokens = input.split(' ');
    let left = tokens.next().unwrap_or("0").trim().parse::<i64>()?;
    let op = tokens.next().unwrap_or("+").trim();
    let right = tokens.next().unwrap_or("0").trim().parse::<i64>()?;

    let operation = match op {
        "+" => Operation::Add(left, right),
        "-" => Operation::Sub(left, right),
        "*" => Operation::Mul(left, right),
        "/" => Operation::Div(left, right),
        _ => panic!("Error while parsing input."),
    };
    Ok(operation)
}

async fn get_result(
    client: &mut CalculatorServiceClient<tonic::transport::Channel>,
    operation: Operation,
) -> Result<i64, Box<dyn Error>> {
    let response = match operation {
        Operation::Add(a, b) => client.add(tonic::Request::new(CalcInput { a, b })).await?,
        Operation::Sub(a, b) => client.sub(tonic::Request::new(CalcInput { a, b })).await?,
        Operation::Mul(a, b) => client.mul(tonic::Request::new(CalcInput { a, b })).await?,
        Operation::Div(a, b) => client.div(tonic::Request::new(CalcInput { a, b })).await?,
    };

    let data = response.get_ref();
    if data.error {
        Err(Box::new(ResultError { operation }))
    } else {
        Ok(data.result)
    }
}

fn get_input() -> String {
    let mut input_string = String::new();
    let _ = match stdin().read_line(&mut input_string) {
        Ok(_) => {}
        Err(_) => return "".to_string(),
    };
    input_string.trim().to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = CalculatorServiceClient::connect("http://localhost:1234").await?;

    loop {
        println!("Enter simple expression (q to exit):");
        let input = get_input();
        if input.eq("q") || input.eq("Q") {
            exit(0)
        }

        let parsed = parse_input(input).expect("Failed to parse input.");

        let result = get_result(&mut client, parsed).await?;

        println!("Result = {}", result);
    }
}
