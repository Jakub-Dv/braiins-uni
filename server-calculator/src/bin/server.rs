use std::task::{Context, Poll};
use tonic::codegen::Service;
use tonic::{transport::Server, Request, Response, Status};
use tower::layer::Layer;

pub mod calculator_service {
    // name of the grpc package
    tonic::include_proto!("calculator_service");
}

use calculator_service::calculator_service_server::{CalculatorService, CalculatorServiceServer};
use calculator_service::{CalcInput, CalcOutput};

#[derive(Default, Clone)]
struct Calculator;

#[tonic::async_trait]
impl CalculatorService for Calculator {
    async fn add(&self, request: Request<CalcInput>) -> Result<Response<CalcOutput>, Status> {
        let result = request.get_ref().a + request.get_ref().b;
        Ok(Response::new(CalcOutput {
            result,
            error: false,
        }))
    }

    async fn sub(&self, request: Request<CalcInput>) -> Result<Response<CalcOutput>, Status> {
        let result = request.get_ref().a - request.get_ref().b;
        Ok(Response::new(CalcOutput {
            result,
            error: false,
        }))
    }

    async fn mul(&self, request: Request<CalcInput>) -> Result<Response<CalcOutput>, Status> {
        let result = request.get_ref().a * request.get_ref().b;
        Ok(Response::new(CalcOutput {
            result,
            error: false,
        }))
    }

    async fn div(&self, request: Request<CalcInput>) -> Result<Response<CalcOutput>, Status> {
        let result = request.get_ref().a / request.get_ref().b;
        Ok(Response::new(CalcOutput {
            result,
            error: false,
        }))
    }
}

#[derive(Clone)]
struct LogService<S> {
    target: &'static str,
    service: S,
}

impl<S, Request> Service<Request> for LogService<S>
where
    S: Service<Request>,
    Request: std::fmt::Debug,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        println!("Request: {:?}, Target: {:?}", request, self.target);
        self.service.call(request)
    }
}

#[derive(Clone)]
struct LogLayer {
    target: &'static str,
}

impl<S> Layer<S> for LogLayer {
    type Service = LogService<S>;

    fn layer(&self, service: S) -> Self::Service {
        Self::Service {
            target: self.target,
            service,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: std::net::SocketAddr = "127.0.0.1:1234".parse().unwrap();
    let calculator = Calculator::default();

    let layer = LogLayer { target: "abcd" };

    Server::builder()
        .layer(layer)
        .add_service(CalculatorServiceServer::new(calculator))
        .serve(addr)
        .await?;

    Ok(())
}
