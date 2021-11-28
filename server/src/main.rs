use std::net::TcpListener;

fn main() {
    let listener: TcpListener: TcpListener = 
            TcpListener::bind(addr: "127.0.0.1:7878").unwrap();
}
