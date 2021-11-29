use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;

fn main() {
    let listener: TcpListener = 
            TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming(){
        let _stream = stream.unwrap();

        handle_connection(_stream);
    }
}

fn handle_connection(mut _stream: TcpStream){
    let mut buffer:[u8; 1024] = [0; 1024]; // this is the limit setter

    _stream.read(&mut buffer).unwrap();

    let _get = b"GET / HTTP/1.1\r\n"; // this is the bit array string

    let (status_line, filename) = 
        if buffer.starts_with(_get) {
            ("HTTP/1.1 200 OK", "index.html")
        } else {
        ("HTTP/1.1 404 NOT FOUND","404.html")
        };

    

    let contents: String = fs::read_to_string(filename).unwrap();

        let _response =  format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );

        _stream.write(_response.as_bytes()).unwrap();
        _stream.flush().unwrap(); 
    

}

//HTTP Status-Code

//headers Crlf

//message-body

