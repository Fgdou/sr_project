use std::{sync::{Arc, Mutex}, thread, time::Duration};
use client::Client;
use websocket::sync::Server;

use crate::{game::Game, objects::{MessageServer, Player}};

mod objects;
mod client;
mod game;

fn handle_loop(game: Arc<Mutex<Game>>) {
    loop {
        thread::sleep(Duration::from_millis(300));
        let _ = game.lock().map(|mut g| g.tick());
    }
}

fn main() {

    let game = Arc::new(Mutex::new(Game::new()));

    let server = Server::bind("0.0.0.0:8080").unwrap();

    println!("Listening to 0.0.0.0:8080");

    let game_copy = game.clone();
    thread::spawn(move || handle_loop(game_copy));

    for request in server.filter_map(Result::ok) {
        let game = game.clone();
        thread::spawn(move || {
            let client = request.accept().unwrap();
            let ip = client.peer_addr().unwrap();

            println!("Connection from {}", ip);

            let (mut receiver, sender) = client.split().unwrap();

            let id = {
                let mut game = game.lock().unwrap();
                let id = game.next_id();

                let mut player = Client::new(Player::new(id), sender);
                player.send_message(&MessageServer::SetId(player.player.get_id()));
                game.add_client(player);
                id
            };


            for message in receiver.incoming_messages() {
                if let Ok(message) = message {
                    let _ = game.lock().map(|mut game| {
                        let res = game.get_client(id).map(|p| p.handle_message(message));
                        if let Some(res) = res {
                            if res == false {
                                game.remove_client(id);
                            }
                        };
                    });
                } else {
                    break;
                }
            }
        });
    }
}
