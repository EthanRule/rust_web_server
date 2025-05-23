use hello::ThreadPool;
use hello::Hardware;

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let listner = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind address. Check if address is already in use.");
    let hardware = Hardware::new();
    let pool = ThreadPool::new(hardware.logical_processors);

    for stream in listner.incoming() {
        let stream = stream.expect("Failed to get stream from TcpListner");

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = match buf_reader.lines().next() {
        Some(Ok(line)) => line,
        Some(Err(e)) => {
            eprintln!("Error reading line: {}", e);
            return;
        }
        None => {
            eprintln!("No lines received in the request.");
            return;
        }
    };

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap_or_else(|_error| {
        eprintln!("file: {} not found", filename);
        String::new()
    });

    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).expect("Failed to write bytes to stream");
}
