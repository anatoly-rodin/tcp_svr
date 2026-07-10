use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::env;
use dotenv::dotenv;
use colored::Colorize;

fn main() {
    dotenv().ok();

    let server_host: String = env::var("SERVER_HOST")
        .expect("The SERVER_HOST parameter is missing from the .env file.");

    let server_port: String = env::var("SERVER_PORT")
        .expect("The SERVER_PORT parameter is missing from the .env file.");

    let server_bind = format!("{server_host}:{server_port}");

    let listener = TcpListener::bind(server_bind)
        .expect("Failed to bind the address.");

    println!("{}", format!("The server is listening on {}:{}", server_host, server_port).green());

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
    println!("{}", format!("Received request: {request}").cyan());

    let response: &[u8] = "Hello, Client!".as_bytes();
    stream.write(response).expect("Failed to write the response!");
}