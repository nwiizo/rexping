use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    loop {
        let (stream, peer_addr) = listener.accept().unwrap();
        println!("{}", peer_addr);
    }
}
