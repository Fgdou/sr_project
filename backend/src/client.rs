use std::net::TcpStream;

use websocket::{sync::Writer, OwnedMessage};

use crate::objects::{Direction, MessageServer, Player, Vector2};

pub struct Client<T> {
    player: Player,
    writer: T,
    next_move: Vec<Direction>,
}

/**
 * Interface created for mocking TcpStream from websocket
 */
pub trait WriterInterface {
    fn send_message(&mut self, message: &OwnedMessage);
}

impl WriterInterface for Writer<TcpStream> {
    fn send_message(&mut self, message: &OwnedMessage) {
        let _ = Writer::send_message(self, message);
    }
}

impl<T: WriterInterface> Client<T> {
    /**
     * Send a message from the server to the client
     */
    pub fn send_message(&mut self, message: &MessageServer) {
        match message {
            MessageServer::Error(error) => println!("Error for {} : {}", self.player.username(), error),
            _ => ()
        }
        self.send_raw_message(&message.into());
    }
    /**
     * Create the client
     */
    pub fn new(player: Player, writer: T) -> Self {
        Self {
            player,
            writer,
            next_move: Vec::new()
        }
    }
    /**
     * get the player
     */
    pub fn player(&self) -> &Player {
        &self.player
    }
    /**
     * get the player
     */
    pub fn player_mut(&mut self) -> &mut Player {
        &mut self.player
    }
    /**
     * Send a websocket message 
     */
    pub fn send_raw_message(&mut self, message: &OwnedMessage) {
        let _ = self.writer.send_message(message);
    }

    /**
     * Get the next move stored in the stack
     */
    fn get_next_move(&mut self) -> Option<Direction> {
        self.next_move.pop()
    }

    /**
     * store the next move sended by the client
     */
    pub fn add_next_move(&mut self, direction: Direction) {
        if self.next_move.len() > 3 {
            self.next_move.pop();
        }
        self.next_move.insert(0, direction);
    }

    /**
     * Handle the game logic
     */
    pub fn update(&mut self, size: &Vector2) {
        if let Some(dir) = self.get_next_move() {
            self.player.set_direction(dir.clone());
        }
        self.player.update(size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Writer;
    impl WriterInterface for Writer {
        fn send_message(&mut self, _: &websocket::OwnedMessage) {}
    }

    fn client_example() -> Client<Writer> {
        Client::new(Player::new(0), Writer{})
    }

    #[test]
    fn player_move_empty() {
        let mut client = client_example();
        assert!(client.get_next_move().is_none());
    }

    #[test]
    fn player_move_one() {
        let mut client = client_example();
        client.add_next_move(Direction::Down);
        assert_eq!(Some(Direction::Down), client.get_next_move());
    }

    #[test]
    fn player_move_five() {
        let mut client = client_example();
        client.add_next_move(Direction::Down);
        client.add_next_move(Direction::Up);
        client.add_next_move(Direction::Left);
        client.add_next_move(Direction::Right);
        client.add_next_move(Direction::Down);
        assert_eq!(Some(Direction::Up), client.get_next_move());
        assert_eq!(Some(Direction::Left), client.get_next_move());
        assert_eq!(Some(Direction::Right), client.get_next_move());
        assert_eq!(Some(Direction::Down), client.get_next_move());
    }
}