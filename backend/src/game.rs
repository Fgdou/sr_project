use rand::Rng;

use crate::{client::Client, objects::{Event, Infos, MessageServer, Player, PlayerState, Vector2}};

pub struct Game {
    players: Vec<Client>,
    size: Vector2,
    apples: Vec<Vector2>,
    last_players: Vec<Player>,
    last_apples: Vec<Vector2>,
    diffs: Vec<Event>,
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
        }
    }
    pub fn get_client(&mut self, id: i32) -> Option<&mut Client> {
        self.players.iter_mut().find(|p| p.player.get_id() == id)
    }
    pub fn add_client(&mut self, mut client: Client) {
        let pos = self.free_space(3);

        if let Some(pos) = pos {
            (0..3).for_each(|i| client.player.add_position(pos.clone() + Vector2::new(0, i)));

            let mut all_players: Vec<Player> = self.players.iter()
                .filter(|p| match p.player.get_state() {
                    PlayerState::Waiting(_) => true,
                    PlayerState::Connecting => false,
                    PlayerState::Dead(0) => false,
                    PlayerState::Dead(_) => true,
                    PlayerState::Running => true,
                })
                .map(|p| p.player.clone()).collect();
            let apples = self.apples.clone();
            client.send_message(&MessageServer::Infos(Infos{
                apples,
                players: all_players,
                size: self.size.clone()
            }));
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
            .filter(|p1| players.iter().any(|p2| p1.player.intersect_player(&p2)))
            .for_each(|p| p.player.kill());

        // apples
        self.apples.retain(|apple| {
            let player = self.players.iter_mut().find(|p| p.player.intersect_apple(apple));
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
            p.send_message(&MessageServer::ChangeInfos(self.diffs.clone()))
        });
        self.diffs.clear();

        // update history
        self.last_apples = self.apples.clone();
        self.last_players = self.players.iter().map(|p| p.player.clone()).collect();
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
}