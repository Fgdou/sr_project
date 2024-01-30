use std::net::TcpStream;

use websocket::{sync::Writer, OwnedMessage};

use crate::objects::{Direction, MessageClient, MessageServer, Player};

pub struct Client {
    pub player: Player,
    pub writer: Writer<TcpStream>,
    pub next_move: Vec<Direction>,
}

impl Client {
    pub fn send_message(&mut self, message: &MessageServer) {
        match message {
            MessageServer::Error(error) => println!("Error for {} : {}", self.player.get_username(), error),
            _ => ()
        }
        let _ = self.writer.send_message::<OwnedMessage>(&message.into());
    }
    pub fn new(player: Player, writer: Writer<TcpStream>) -> Self {
        Self {
            player,
            writer,
            next_move: Vec::new()
        }
    }
    pub fn handle_message(&mut self, message: OwnedMessage) -> bool {
        match message {
            OwnedMessage::Close(_) => {
                let message = OwnedMessage::Close(None);
                let _ = self.writer.send_message(&message);
                println!("Client {}:{} disconnected", self.player.get_id(), self.player.get_username());
                return false;
            }
            OwnedMessage::Ping(ping) => {
                let message = OwnedMessage::Pong(ping);
                let _ = self.writer.send_message(&message);
            }
            OwnedMessage::Text(value) => {
                let message = serde_json::from_str(value.as_str());
                match message {
                    Ok(MessageClient::Connection(pseudo)) => {
                        let pseudo = pseudo.trim();
                        if pseudo.len() > 20 {
                            self.send_message(&MessageServer::Error("Username should be less than 20 characters".to_string()))
                        } else if pseudo.chars().any(|c| !c.is_alphanumeric()) {
                            self.send_message(&MessageServer::Error("Username should be only numbers and chars in ASCII".to_string()))
                        } else {
                            self.player.set_username(pseudo.to_string());
                        }
                    },
                    Ok(MessageClient::ChangeDirection(direction)) => {
                        if self.next_move.len() > 3 {
                            self.next_move.pop();
                        }
                        self.next_move.insert(0, direction);
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