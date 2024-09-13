use core::str;
use std::{io::{Read, Write}, net::TcpStream};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();
    stream.write_all("Hello".as_bytes());
    let mut buffer = [0; 5];
    stream.read_exact(&mut buffer).unwrap();
    println!(
        "Got response from server:{:?}",
        str::from_utf8(&buffer).unwrap()
    )
}
