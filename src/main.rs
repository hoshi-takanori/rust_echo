use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("localhost:9000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}
