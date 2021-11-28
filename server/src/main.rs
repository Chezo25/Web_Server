use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

fn main() {
    let listener: TcpListener = 
            TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming(){
        let _stream = stream.unwrap();

        handle_connection(_stream);
    }
}

fn handle_connection(mut _stream: TcpStream){
    let mut buffer:[u8; 1024] = [0; 1024];

    _stream.read(&mut buffer).unwrap();
    println!("Requst: {}",
             String::from_utf8_lossy(&buffer[..])
);
}