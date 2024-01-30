use std::ops::IndexMut;

use rand::Rng;

use crate::{client::Client, objects::{Infos, MessageServer, Player, Vector2}};

pub struct Game {
    players: Vec<Client>,
    size: Vector2,
    apples: Vec<Vector2>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            players: Vec::new(),
            size: Vector2::new(20, 20),
            apples: Vec::new()
        }
    }
    pub fn get_client(&mut self, id: i32) -> Option<&mut Client> {
        self.players.iter_mut().find(|p| p.player.id == id)
    }
    pub fn get_player(&mut self, id: i32) -> Option<&mut Player> {
        self.get_client(id).map(|p| &mut p.player)
    }
    pub fn add_client(&mut self, mut client: Client) {
        let mut rng = rand::thread_rng();
        let pos = Vector2{
            x: rng.gen_range(0..self.size.x),
            y: rng.gen_range(5..self.size.y),
        };
        (0..3).for_each(|_| client.player.positions.push(pos.clone()));
        self.players.push(client)
    }
    pub fn next_id(&self) -> i32 {
        (0..i32::MAX).into_iter().find(|i| self.players.iter().all(|p| &p.get_id() != i)).unwrap()
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
                self.apples.push(pos);
            }
        }

        // players
        self.players.iter_mut().for_each(|p| p.player.update(&self.size));

        // collision
        let players: Vec<Player> = self.players.iter().map(|p| p.player.clone()).collect();
        self.players.iter_mut()
            .filter(|p1| players.iter().any(|p2| p1.player.intersect_player(&p2)))
            .for_each(|p| p.player.kill());

        // apples
        self.apples.retain(|apple| {
            let player = self.players.iter_mut().find(|p| p.player.positions.contains(apple));
            if let Some(player) = player {
                player.player.increase();
                false
            } else {
                true
            }
        });

        // send message
        let all_players: Vec<Player> = self.players.iter().map(|p| p.player.clone()).collect();
        let apples = self.apples.clone();
        self.players.iter_mut().for_each(|p| {
            p.send_message(&MessageServer::Infos(Infos{
                apples: apples.clone(),
                players: all_players.clone(),
                size: self.size.clone()
            }))
        })
    }
    pub fn remove_client(&mut self, id: i32) {
        self.players.retain(|p| p.get_id() != id);
    }
}