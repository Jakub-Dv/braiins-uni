use hyper::body::HttpBody;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use lazy_static::lazy_static;
use prometheus::{register_int_counter, Encoder, IntCounter, TextEncoder};
use std::{convert::Infallible, net::SocketAddr};

lazy_static! {
    static ref HTTP_REQUEST_TOTAL: IntCounter = register_int_counter!(
        "http_requests_total",
        "the total number of HTTP requests made"
    )
    .unwrap();
    static ref HTTP_RESPONSE_SIZE_TOTAL: IntCounter = register_int_counter!(
        "http_response_size_total",
        "the HTTP response sizes in bytes"
    )
    .unwrap();
}

fn record_request_ok(_: &Request<Body>) {
    HTTP_REQUEST_TOTAL.inc();
}

fn record_response_ok(response: &Response<Body>) {
    let body = response.body();
    let value = body.size_hint().exact().unwrap();

    HTTP_RESPONSE_SIZE_TOTAL.inc_by(value);
}

fn get_prometheus_data() -> Vec<u8> {
    let metrics = prometheus::gather();
    let encoder = TextEncoder::new();
    let mut buf = vec![];
    encoder.encode(&metrics, &mut buf).unwrap();
    buf
}

async fn handle(request: Request<Body>) -> Result<Response<Body>, Infallible> {
    record_request_ok(&request);
    let data = get_prometheus_data();
    let response = Response::builder()
        .status(200)
        .body(Body::from(data))
        .unwrap();
    record_response_ok(&response);
    Ok(response)
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 9898));

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Serving HTTP server on {}", addr);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
