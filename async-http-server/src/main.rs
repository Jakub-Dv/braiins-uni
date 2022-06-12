use smol::{fs, io, net, prelude::*};

#[smol_potat::main]
async fn main() -> io::Result<()> {
    // Listen for incoming TCP connections on localhost port 7878
    let listener = net::TcpListener::bind("127.0.0.1:7878").await?;

    loop {
        let (stream, _) = listener.accept().await?;

        handle_connection(stream).await
    }
}

async fn handle_connection(mut stream: net::TcpStream) {
    // Read the first 1024 bytes of data from the stream
    let mut buffer = [0; 1024];
    stream
        .read(&mut buffer)
        .await
        .expect("Failed to read message");

    let get = b"GET / HTTP/1.1\r\n";

    // Respond with greetings or a 404,
    // depending on the data in the request
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = fs::read_to_string(filename)
        .await
        .expect(&*format!("Failed to read a file {}", filename));

    // Write response back to the stream,
    // and flush the stream to ensure the response is sent back to the client
    let response = format!("{}{}", status_line, contents);
    stream
        .write_all(response.as_bytes())
        .await
        .expect("Failed to write message");
    stream.flush().await.expect("Failed to flush message");
}
