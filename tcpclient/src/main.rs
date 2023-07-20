use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    println!("\n ---- Connection established! ----");
    stream.write("Hello".as_bytes()).unwrap();
    println!("\"Hello\" send to the echo server!\n");
    let mut buffer = [0; 5];
    stream.read(&mut buffer).unwrap();
    println!(
        "Response from server: {:?}",
        str::from_utf8(&buffer).unwrap()
    );
}