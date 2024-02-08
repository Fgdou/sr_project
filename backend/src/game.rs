use std::net::TcpStream;

use websocket::{sync::Writer, OwnedMessage};

use crate::{client, objects::{Event, Infos, MessageClient, MessageServer, Player, PlayerState, Vector2}};

type Client = client::Client<Writer<TcpStream>>;

pub struct Game {
    clients: Vec<Client>,
    size: Vector2,
    apples: Vec<Vector2>,
    last_players: Vec<Player>,
    last_apples: Vec<Vector2>,
    diffs: Vec<Event>,
    message_count: u32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            clients: Vec::new(),
            size: Vector2::new(30, 30),
            apples: Vec::new(),
            last_apples: Vec::new(),
            last_players: Vec::new(),
            diffs: Vec::new(),
            message_count: 0,
        }
    }
    pub fn number_players(&self) -> i32 {
        self.clients.len() as i32
    }
    pub fn get_client(&mut self, id: i32) -> Option<&mut Client> {
        self.clients.iter_mut().find(|p| p.player().id() == id)
    }
    pub fn get_infos(&self) -> Infos {
        let all_players: Vec<Player> = self.clients.iter()
            .map(|p| p.player().clone()).collect();
        let apples = self.apples.clone();

        Infos{
            apples,
            players: all_players,
            size: self.size.clone(),
            message_count: self.message_count
        }
    }
    pub fn add_client(&mut self, mut client: Client) {
        let pos = self.get_free_space(3);

        if let Some(pos) = pos {
            (0..3).for_each(|i| client.player_mut().add_position(pos.clone() + Vector2::new(0, i)));

            client.send_message(&MessageServer::Infos(self.get_infos()));
            self.diffs.push(Event::AddPlayer(client.player().clone()));
            self.clients.push(client);
        } else {
            client.send_message(&MessageServer::Error("No space available".to_string()));
        }        
    }
    pub fn next_id(&self) -> i32 {
        (0..i32::MAX).into_iter().find(|i| self.clients.iter().all(|p| &p.player().id() != i)).unwrap_or(0)
    }
    fn tick_apple(&mut self) {
        while self.apples.len() < 10 {
            let pos = Vector2::rand(&self.size);
            if self.apples.iter().all(|a| a != &pos) {
                self.apples.push(pos.clone());
                self.diffs.push(Event::AddApple(pos));
            }
        }
    }
    pub fn players_running(&self) -> Vec<&Player> {
        self.clients.iter()
            .map(|client| client.player())
            .filter(|player| player.state() == &PlayerState::Running)
            .collect()
    }
    pub fn players_running_mut(&mut self) -> Vec<&mut Player> {
        self.clients.iter_mut()
            .map(|client| client.player_mut())
            .filter(|player| player.state() == &PlayerState::Running)
            .collect()
    }
    fn tick_players(&mut self) {
        // players
        self.clients.iter_mut().for_each(|client| {
            client.update(&self.size);
        });

        // collision
        let players: Vec<Player> = self.players_running()
            .iter()
            .map(|p| (*p).clone())
            .collect();

        self.players_running_mut()
            .iter_mut()
            .filter(|p1| players.iter().any(|p2| p1.intersect_player(&p2)))
            .for_each(|p| p.kill());
    }
    fn tick_players_apples(&mut self) {
        let apples = self.apples.clone();
        self.apples = apples
            .into_iter()
            .filter(|apple| {
                if let Some(player) = self.players_running_mut()
                    .iter_mut()
                    .find(|p| p.intersect_apple(apple))
                {
                    player.increase();
                    self.diffs.push(Event::RemoveApple((*apple).clone()));
                    false
                } else {
                    true
                }
            })
            .collect();
    }
    fn tick_send_changes(&mut self) {
        self.diffs.extend(self.clients.iter_mut().map(|p| p.player_mut().diff()).flatten());
        self.clients.iter_mut().for_each(|p| {
            p.send_message(&MessageServer::ChangeInfos{events: self.diffs.clone(), count: self.message_count})
        });
        self.diffs.clear();
    }
    pub fn tick(&mut self) {
        self.tick_apple();        
        self.tick_players();
        self.tick_players_apples();
        self.tick_send_changes();        

        // update history
        self.last_apples = self.apples.clone();
        self.last_players = self.clients.iter().map(|p| p.player().clone()).collect();
        self.message_count = self.message_count.wrapping_add(1);
    }
    pub fn remove_client(&mut self, id: i32) {
        self.clients.retain(|p| p.player().id() != id);
        self.diffs.push(Event::RemovePlayer(id))
    }
    pub fn get_free_space(&self, radius: i32) -> Option<Vector2> {
        (0..1000).into_iter().find_map(|_| {
            let pos = Vector2::rand(&self.size);

            let founded = (-radius..=radius).zip(-radius..=radius)
                .into_iter()
                .map(|(x, y)| pos.clone() + Vector2::new(x, y))
                .any(|p| 
                        !(p.x >= 0 && p.y >= 0 && p.x < self.size.x && p.y < self.size.y) || 
                        self.clients.iter().any(|player| player.player().intersect(&p)));

            if founded {
                None
            } else {
                return Some(pos)
            }
        })
    }
    fn handle_client_message(&mut self, message: MessageClient, player_id: i32) {
        match message {
            MessageClient::Connection(pseudo) => {
                let pseudo = self.check_pseudo(pseudo);

                let client = self.get_client(player_id).unwrap();

                match pseudo {
                    Ok(pseudo) => client.player_mut().set_username(pseudo),
                    Err(error) => client.send_message(&MessageServer::Error(error)),
                }
            },
            MessageClient::ChangeDirection(direction) => {
                let client = self.get_client(player_id).unwrap();
                client.add_next_move(direction)
            },
            MessageClient::ResendAll => {
                let infos = self.get_infos();
                let client = self.get_client(player_id).unwrap();
                client.send_message(&MessageServer::Infos(infos))
            }
            
        }
    }
    fn check_pseudo(&self, name: String) -> Result<String, String> {
        let pseudo = name.trim();
        let existing_players: Vec<String> = self.clients.iter().map(|p| p.player().username().clone()).collect();
        if pseudo.len() > 10 || pseudo.len() < 4 {
            Err("Username should be between 4 and 10 characters".to_string())
        } else if pseudo.chars().any(|c| !c.is_alphanumeric()) {
            Err("Username should be only numbers and letters".to_string())
        } else if existing_players.contains(&pseudo.to_string()) {
            Err("Username already exists".to_string())
        } else {
            Ok(pseudo.to_string())
        }
    }
    pub fn handle_message(&mut self, message: OwnedMessage, player_id: i32) {
        if self.get_client(player_id).is_none() {
            println!("Discard {}", player_id);
            return
        }
        match message {
            OwnedMessage::Close(_) => {
                let number_players = self.clients.len();
                let client = self.get_client(player_id).unwrap();
                let message = OwnedMessage::Close(None);
                let _ = client.send_raw_message(&message);
                println!("Client {}:{} disconnected : {} players", client.player().id(), client.player().username(), number_players-1);
                return;
            }
            OwnedMessage::Ping(ping) => {
                let client = self.get_client(player_id).unwrap();
                let message = OwnedMessage::Pong(ping);
                let _ = client.send_raw_message(&message);
            }
            OwnedMessage::Text(value) => {
                let message = serde_json::from_str(value.as_str());
                match message {
                    Ok(message) => self.handle_client_message(message, player_id),
                    Err(_) => {
                        let client = self.get_client(player_id).unwrap();
                        client.send_message(&MessageServer::Error("Fail to interpret message".to_string()));
                    }
                }
            }
            _ => ()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_free_space_empty(){
        let mut game = Game::new();
        game.size = Vector2::zero();

        assert_eq!(None, game.get_free_space(0))
    }

    #[test]
    fn get_free_space_one_zero(){
        let mut game = Game::new();
        game.size = Vector2::new(1, 1);

        assert_eq!(Some(Vector2::new(0,0)), game.get_free_space(0));
        assert_eq!(None, game.get_free_space(1));
    }

    #[test]
    fn get_free_space(){
        let mut game = Game::new();
        game.size = Vector2::new(5, 5);

        assert_eq!(Some(Vector2::new(2,2)), game.get_free_space(2));
    }

    #[test]
    fn check_pseudo_size() {
        let game = Game::new();

        assert!(game.check_pseudo("Hey".to_string()).is_err());
        assert!(game.check_pseudo("Heyy".to_string()).is_ok());
        assert!(game.check_pseudo("abcdefoiwu".to_string()).is_ok());
        assert!(game.check_pseudo("abcdefoiwui".to_string()).is_err());
        assert!(game.check_pseudo("abcdefoiwuifdsifjogjgoidfjiofdjhoifg".to_string()).is_err());
    }

    #[test]
    fn check_pseudo_special_char() {
        let game = Game::new();

        assert!(game.check_pseudo("Heyy".to_string()).is_ok());
        assert!(game.check_pseudo("hey146".to_string()).is_ok());
        assert!(game.check_pseudo("1355".to_string()).is_ok());
        assert!(game.check_pseudo(" 1355 ".to_string()).is_ok());
        assert!(game.check_pseudo("Hey!".to_string()).is_err());
        assert!(game.check_pseudo("He ee".to_string()).is_err());
        assert!(game.check_pseudo("He_ee".to_string()).is_err());
        assert!(game.check_pseudo("He-ee".to_string()).is_err());
    }
}