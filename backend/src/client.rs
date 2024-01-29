use std::net::TcpStream;

use websocket::{sync::Writer, OwnedMessage};

use crate::objects::{MessageClient, MessageServer, Player, PlayerState};

pub struct Client {
    pub player: Player,
    pub writer: Writer<TcpStream>
}

impl Client {
    pub fn get_id(&self) -> i32 {
        self.player.id
    }
    pub fn send_message(&mut self, message: &MessageServer) {
        let _ = self.writer.send_message::<OwnedMessage>(&message.into());
    }
    pub fn handle_message(&mut self, message: OwnedMessage) -> bool {
        match message {
            OwnedMessage::Close(_) => {
                let message = OwnedMessage::Close(None);
                self.writer.send_message(&message).unwrap();
                println!("Client {} disconnected", self.get_id());
                return false;
            }
            OwnedMessage::Ping(ping) => {
                let message = OwnedMessage::Pong(ping);
                self.writer.send_message(&message).unwrap();
            }
            OwnedMessage::Text(value) => {
                let message: MessageClient = serde_json::from_str(value.as_str()).expect("Not a message");
                match message {
                    MessageClient::Connection(pseudo) => {
                        self.player.username = pseudo;
                        self.player.state = PlayerState::Running;
                    },
                    MessageClient::ChangeDirection(direction) => {
                        self.player.direction = direction;
                    },
                    _ => {
                        self.send_message(&MessageServer::Error("Fail to interpret message".to_string()))
                    }
                }
            }
            _ => ()
        }
        true
    }
}