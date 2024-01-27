use crate::{client::Client, objects::{Infos, MessageServer, Player, Vector2}};

pub struct Game {
    players: Vec<Client>,
    size: Vector2
}

impl Game {
    pub fn new() -> Self {
        Self {
            players: Vec::new(),
            size: Vector2::new(100, 100)
        }
    }
    pub fn get_client(&mut self, id: i32) -> Option<&mut Client> {
        self.players.iter_mut().find(|p| p.player.id == id)
    }
    pub fn get_player(&mut self, id: i32) -> Option<&mut Player> {
        self.get_client(id).map(|p| &mut p.player)
    }
    pub fn add_client(&mut self, client: Client) {
        self.players.push(client)
    }
    pub fn next_id(&self) -> i32 {
        (0..i32::MAX).into_iter().find(|i| self.players.iter().all(|p| &p.get_id() != i)).unwrap()
    }
    pub fn tick(&mut self) {
        let all_players: Vec<Player> = self.players.iter().map(|p| p.player.clone()).collect();
        self.players.iter_mut().for_each(|p| {
            p.send_message(&MessageServer::Infos(Infos{
                apples: vec!(Vector2::new(10, 10)),
                players: all_players.clone(),
                size: self.size.clone()
            }))
        })
    }
}