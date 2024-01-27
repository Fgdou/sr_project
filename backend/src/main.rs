use std::thread;
use websocket::sync::Server;
use websocket::OwnedMessage;
use serde_json;

use crate::objects::MessageClient;

mod objects;

fn main() {
    let server = Server::bind("0.0.0.0:8080").unwrap();

    println!("Listening to 0.0.0.0:8080");

    for request in server.filter_map(Result::ok) {
        thread::spawn(|| {
            let mut client = request.accept().unwrap();
            let ip = client.peer_addr().unwrap();

            println!("Connection from {}", ip);

            let (mut receiver, mut sender) = client.split().unwrap();

            for message in receiver.incoming_messages() {
                let message = message.unwrap();

                match message {
                    OwnedMessage::Close(_) => {
                        let message = OwnedMessage::Close(None);
                        sender.send_message(&message).unwrap();
                        println!("Client {} disconnected", ip);
                        return;
                    }
                    OwnedMessage::Ping(ping) => {
                        let message = OwnedMessage::Pong(ping);
                        sender.send_message(&message).unwrap();
                    }
                    OwnedMessage::Text(value) => {
                        println!("Message : {}", value);
                        let message: MessageClient = serde_json::from_str(value.as_str()).expect("Not a message");
                        println!("Message: {:?}", &message);
                    }
                    _ => ()
                }
            }
        });
    }
}
