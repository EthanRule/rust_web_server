// TODO: Migrate this to use axum.
use axum::{Router, routing::get};

use hello::Hardware;
use std::error::Error;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let hardware = Hardware::new();
    let listner = TcpListener::bind("127.0.0.1:7878")
        .await
        .expect("Failed to bind address. Check if address is already in use.");

    loop {
        let (stream, _) = listner
            .accept()
            .await
            .expect("Failed to get stream from TcpListener");
        tokio::spawn(async move {
            handle_connection(stream).await;
        });
    }
}

async fn send_recv() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    println!("sending...");
    stream.write_all(b"hello world!").await?;
    let mut buf = vec![0; 1024];
    let n = stream.read(&mut buf).await?;
    println!("received: {:?}", n);
    Ok(())
}

// Listen for commands here
async fn handle_connection(mut stream: TcpStream) {
    println!("new connection!");
    let buf_reader = BufReader::new(&mut stream);
    let mut lines = buf_reader.lines();
    let request_line = match lines.next_line().await {
        Ok(Some(line)) => line,
        Ok(None) => {
            eprintln!("No lines received in the request.");
            return;
        }
        Err(e) => {
            eprintln!("Error reading line: {}", e);
            return;
        }
    };

    // Send request_line to Core and wait (needs to act as a client here by connecting to Core)
    // Connect to IPV6, 8080.
    let _ = send_recv().await;

    // Receive status update from Core and continue

    // let (status_line, filename) = match &request_line[..] {
    //     "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
    //     "GET /sleep HTTP/1.1" => {
    //         thread::sleep(Duration::from_secs(5));
    //         ("HTTP/1.1 200 OK", "hello.html")
    //     }
    //     _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    // };

    // let contents = fs::read_to_string(filename).unwrap_or_else(|_error| {
    //     eprintln!("file: {} not found", filename);
    //     String::new()
    // });

    // let length = contents.len();
    // let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    // stream
    //     .write_all(response.as_bytes())
    //     .await
    //     .expect("Failed to write bytes to stream");
}
