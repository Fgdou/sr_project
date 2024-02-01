use std::net::TcpStream;

use websocket::{sync::Writer, OwnedMessage};

use crate::objects::{Direction, MessageServer, Player};

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
}