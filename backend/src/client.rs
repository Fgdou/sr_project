use std::net::TcpStream;

use websocket::{sync::Writer, OwnedMessage};

use crate::objects::{Direction, MessageServer, Player};

pub struct Client {
    player: Player,
    writer: Writer<TcpStream>,
    next_move: Vec<Direction>,
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
    pub fn player(&self) -> &Player {
        &self.player
    }
    pub fn player_mut(&mut self) -> &mut Player {
        &mut self.player
    }
    pub fn send_raw_message(&mut self, message: &OwnedMessage) {
        let _ = self.writer.send_message(message);
    }
    pub fn get_next_move(&mut self) -> Option<Direction> {
        self.next_move.pop()
    }
    pub fn add_next_move(&mut self, direction: Direction) {
        if self.next_move.len() > 3 {
            self.next_move.pop();
        }
        self.next_move.insert(0, direction);
    }
}