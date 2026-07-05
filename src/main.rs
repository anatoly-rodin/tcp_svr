use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .expect("Failed to bind the address.");

    println!("The server is listening on 127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(curr_stream) => {
                std::thread::spawn(|| handle_client(curr_stream));
            },
            Err(e) => {
                eprintln!("Failed to establish connection: {e}");
            },
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];

    stream.read(&mut buffer).expect("Failed to read from the client");

    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request: {request}");

    let response: &[u8] = "Hello, Client!".as_bytes();
    stream.write(response).expect("Failed to write the response!");
}