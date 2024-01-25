use tokio::net::TcpListener;
use web_socket::*;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let mut listener = TcpListener::bind("0.0.0.0:8080").await;
    println!("Websocket server started on ws://0.0.0.0:8080");

    loop {
    }
}
