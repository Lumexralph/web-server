use web_server::ThreadPool;

use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use std::thread;
use std::time::Duration;

fn main() {
    // bind the TCP connection to address and port
    // unwrap the Result, Ok and panics if there is Err
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down");
}


// handle the data sent in the stream from the client
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK \r\n\r\n", "home.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(10));
        ("HTTP/1.1 200 OK \r\n\r\n", "home.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND \r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let response = format!("{}{}", status_line, contents);

    // create the response to the client
    stream.write(response.as_bytes()).unwrap();

    // flush will wait and prevent the program from continuing until
    // all the bytes are written to the connection
    stream.flush().unwrap();
}
