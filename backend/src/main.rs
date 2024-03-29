use std::{net::TcpStream, sync::{Arc, Mutex}, thread, time::Duration};
use chrono::Local;
use client::Client;
use websocket::{sync::{Server, Writer}, OwnedMessage};

use crate::objects::{MessageServer, Player, Vector2};

type Game = game::Game<Writer<TcpStream>>;

mod objects;
mod client;
mod game;
mod storage;

/**
 * main thread that handle the game loop
 */
fn handle_loop(game: Arc<Mutex<Game>>) {
    let mut last = Local::now() - Duration::from_millis(300);
    loop {
        let now = Local::now();
        let diff_ms = now.signed_duration_since(last).num_milliseconds();

        let waittime = 300 - diff_ms;

        // println!("{}", waittime);

        if waittime > 0 {
            thread::sleep(Duration::from_millis(waittime as u64));
        }

        last = Local::now();

        
        let _ = game.lock().map(|mut g| g.tick());
    }
}

/**
 * Handle a new player connection
 */
fn handle_new_client(game: Arc<Mutex<Game>>, ws_client: websocket::client::sync::Client<TcpStream>) {
    let (mut receiver, mut sender) = ws_client.split().unwrap();

    // Get an id for the player
    let id = {
        match game.lock() {
            Ok(mut game) => {
                let id = game.next_id();

                let mut game_client = Client::new(Player::new(id), sender);
                game_client.send_message(&MessageServer::SetId(id));
                game.add_client(game_client);
                id
            },
            Err(_) => {
                println!("Error while getting an id");
                let message = &MessageServer::Error("Error while getting an id".to_string());

                let _ = sender.send_message(&OwnedMessage::Text(serde_json::to_string(message).unwrap()));
                let _ = sender.shutdown();
                return;  
            },
        }
    };


    // Loop waiting for messages
    for message in receiver.incoming_messages() {
        if let Ok(message) = message {
            // message
            let _ = game.lock().map(|mut game| {
                game.handle_message(message, id);
            });
        } else {
            // disconnection
            let _ = game.lock().map(|mut g| g.remove_client(id));
            break;
        }
    }
}

fn main() {

    let game = Arc::new(Mutex::new(Game::new(Vector2::new(30, 30))));

    let server = Server::bind("0.0.0.0:8080").unwrap();

    println!("Listening to 0.0.0.0:8080");

    let game_ref_copy = game.clone();
    thread::spawn(move || handle_loop(game_ref_copy));

    for request in server.filter_map(Result::ok) {
        let game = game.clone();

        // create thread for the client
        thread::spawn(move || {
            let client = request.accept().unwrap();
            let ip = client.peer_addr().unwrap();

            println!("Connection from {} : {} players", ip, game.lock().map(|g| g.number_players()).unwrap_or(0)+1);

            handle_new_client(game, client)
        });
    }
}
