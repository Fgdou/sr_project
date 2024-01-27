use std::{sync::{Arc, Mutex}, thread, time::Duration};
use client::Client;
use websocket::sync::Server;

use crate::{game::Game, objects::Player};

mod objects;
mod client;
mod game;

fn handle_loop(game: Arc<Mutex<Game>>) {
    loop {
        thread::sleep(Duration::from_millis(1000));
        game.lock().unwrap().tick();
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

            let id = game.lock().unwrap().next_id();

            let player = Client {
                player: Player::new(id),
                writer: sender
            };
            game.lock().unwrap().add_client(player);

            for message in receiver.incoming_messages() {
                let message = message.unwrap();

                game.lock().unwrap().get_client(id).unwrap().handle_message(message);
            }
        });
    }
}
