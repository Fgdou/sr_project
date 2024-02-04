use rand::Rng;
use websocket::OwnedMessage;

use crate::{client::Client, objects::{Event, Infos, MessageClient, MessageServer, Player, PlayerState, Vector2}};

pub struct Game {
    players: Vec<Client>,
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
            players: Vec::new(),
            size: Vector2::new(30, 30),
            apples: Vec::new(),
            last_apples: Vec::new(),
            last_players: Vec::new(),
            diffs: Vec::new(),
            message_count: 0,
        }
    }
    pub fn get_client(&mut self, id: i32) -> Option<&mut Client> {
        self.players.iter_mut().find(|p| p.player.get_id() == id)
    }
    pub fn get_infos(&self) -> Infos {
        let all_players: Vec<Player> = self.players.iter()
            .map(|p| p.player.clone()).collect();
        let apples = self.apples.clone();

        Infos{
            apples,
            players: all_players,
            size: self.size.clone(),
            message_count: self.message_count
        }
    }
    pub fn add_client(&mut self, mut client: Client) {
        let pos = self.free_space(3);

        if let Some(pos) = pos {
            (0..3).for_each(|i| client.player.add_position(pos.clone() + Vector2::new(0, i)));

            client.send_message(&MessageServer::Infos(self.get_infos()));
            self.diffs.push(Event::AddPlayer(client.player.clone()));
            self.players.push(client);
        } else {
            client.send_message(&MessageServer::Error("No space available".to_string()));
        }        
    }
    pub fn next_id(&self) -> i32 {
        (0..i32::MAX).into_iter().find(|i| self.players.iter().all(|p| &p.player.get_id() != i)).unwrap_or(0)
    }
    pub fn tick(&mut self) {
        // apples
        while self.apples.len() < 10 {
            let mut rng = rand::thread_rng();
            let pos = Vector2{
                x: rng.gen_range(0..self.size.x),
                y: rng.gen_range(0..self.size.y),
            };
            if self.apples.iter().all(|a| a != &pos) {
                self.apples.push(pos.clone());
                self.diffs.push(Event::AddApple(pos));
            }
        }

        // players
        self.players.iter_mut().for_each(|p| {
            if let Some(dir) = p.next_move.pop() {
                p.player.set_direction(dir.clone());
            }
            p.player.update(&self.size)
        });

        // collision
        let players: Vec<Player> = self.players.iter()
            .filter(|p| p.player.get_state() == &PlayerState::Running)
            .map(|p| p.player.clone()).collect();
        self.players.iter_mut()
            .filter(|p| p.player.get_state() == &PlayerState::Running)
            .filter(|p1| players.iter().any(|p2| p1.player.intersect_player(&p2)))
            .map(|p| p.player.kill())
            .flatten()
            .for_each(|apple| {
                self.apples.push(apple.clone());
                self.diffs.push(Event::AddApple(apple));
            });


        // apples
        self.apples.retain(|apple| {
            let player = self.players.iter_mut()
                .filter(|p| p.player.get_state() == &PlayerState::Running)
                .find(|p| p.player.intersect_apple(apple));
            if let Some(player) = player {
                player.player.increase();
                self.diffs.push(Event::RemoveApple(apple.clone()));
                false
            } else {
                true
            }
        });

        // send message
        self.diffs.extend(self.players.iter_mut().map(|p| p.player.diff()).flatten());
        self.players.iter_mut().for_each(|p| {
            p.send_message(&MessageServer::ChangeInfos{events: self.diffs.clone(), count: self.message_count})
        });
        self.diffs.clear();

        // update history
        self.last_apples = self.apples.clone();
        self.last_players = self.players.iter().map(|p| p.player.clone()).collect();
        self.message_count = self.message_count.wrapping_add(1);
    }
    pub fn remove_client(&mut self, id: i32) {
        self.players.retain(|p| p.player.get_id() != id);
        self.diffs.push(Event::RemovePlayer(id))
    }
    pub fn free_space(&self, radius: i32) -> Option<Vector2> {
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            let x = rng.gen_range(0..self.size.x);
            let y = rng.gen_range(0..self.size.y);

            let mut founded = false;
            for x in x-radius..=x+radius {
                for y in y-radius..=y+radius {
                    if x < 0 || y < 0 || x >= self.size.x || y >= self.size.y {
                        founded = true;
                        continue;
                    }

                    let pos = Vector2::new(x, y);

                    if self.players.iter().any(|p| p.player.get_positions().iter().any(|p| p == &pos)) {
                        founded = true;
                    }
                }
            }
            if !founded {
                return Some(Vector2::new(x, y));
            }
        }
        None
    }
    pub fn handle_message(&mut self, message: OwnedMessage, player_id: i32) {
        match message {
            OwnedMessage::Close(_) => {
                let client = self.get_client(player_id).unwrap();
                let message = OwnedMessage::Close(None);
                let _ = client.writer.send_message(&message);
                println!("Client {}:{} disconnected", client.player.get_id(), client.player.get_username());
                return;
            }
            OwnedMessage::Ping(ping) => {
                let client = self.get_client(player_id).unwrap();
                let message = OwnedMessage::Pong(ping);
                let _ = client.writer.send_message(&message);
            }
            OwnedMessage::Text(value) => {
                let message = serde_json::from_str(value.as_str());
                match message {
                    Ok(MessageClient::Connection(pseudo)) => {
                        let existing_players: Vec<String> = self.players.iter().map(|p| p.player.get_username().clone()).collect();

                        let client = self.get_client(player_id).unwrap();
                        let pseudo = pseudo.trim();
                        if pseudo.len() > 10 || pseudo.len() < 4 {
                            client.send_message(&MessageServer::Error("Username should be between 4 and 10 characters".to_string()))
                        } else if pseudo.chars().any(|c| !c.is_alphanumeric()) {
                            client.send_message(&MessageServer::Error("Username should be only numbers and letters".to_string()))
                        } else if existing_players.contains(&pseudo.to_string()) {
                            client.send_message(&MessageServer::Error("Username already exists".to_string()))
                        } else {
                            client.player.set_username(pseudo.to_string());
                        }
                    },
                    Ok(MessageClient::ChangeDirection(direction)) => {
                        let client = self.get_client(player_id).unwrap();
                        if client.next_move.len() > 3 {
                            client.next_move.pop();
                        }
                        client.next_move.insert(0, direction);
                    },
                    Ok(MessageClient::ResendAll) => {
                        let infos = self.get_infos();
                        let client = self.get_client(player_id).unwrap();
                        client.send_message(&MessageServer::Infos(infos))
                    }
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