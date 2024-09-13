use std::{io::{Read, Write}, net::TcpListener};

fn main() {
    let connection_listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000");
    for stream in connection_listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established");
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    println!("No data received, connection closed by client.");
                    continue;
                }

                println!("Read {} bytes from the client: {:?}", bytes_read, &buffer[..bytes_read]);

                // Echo the data back to the client
                stream.write_all(&buffer[..bytes_read]).unwrap();
            }
            Err(e) => println!("Failed to read from the stream: {}", e),
        }
        stream.write_all(&buffer).unwrap();
    }
}
